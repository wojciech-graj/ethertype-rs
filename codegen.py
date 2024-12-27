import csv
import io
import re
import subprocess
from collections import defaultdict
from itertools import groupby

import requests


def iana(inp, out):
    reader = csv.DictReader(inp)
    out.write(
        "#![allow(unreachable_patterns)]\n"
        "\n"
        "use crate::EtherType;\n"
        "\n"
        "impl EtherType {\n"
        "    /// A brief textual description of the [`EtherType`] sourced from [IANA](https://www.iana.org/assignments/ieee-802-numbers/ieee-802-numbers.xhtml).\n"
        "    pub const fn iana_description(self) -> Option<&'static str> {\n"
        "        Some(match self.0 {\n")
    for row in reader:
        ethertype = row["Ethertype (hex)"].split('-')
        ethertype_pat = f"0x{ethertype[0]}" if len(
            ethertype) == 1 else f"0x{ethertype[0]}..=0x{ethertype[1]}"
        description = row["Description"]
        out.write(f'            {ethertype_pat} => r#"{description}"#,\n')
    out.write("            _ => return None,\n"
              "        })\n"
              "    }\n"
              "}\n")


def format_patterns(assignments):
    lines = []
    for value, ethertypes in assignments.items():
        ethertypes = list(set(ethertypes))
        ethertypes.sort()
        ethertype_pats = []

        for _, g in groupby(enumerate(ethertypes), lambda ix: ix[0] - ix[1]):
            g = list(g)
            ethertype_pats.append(f"0x{g[0][1]:04X}" if len(g) ==
                                  1 else f"0x{g[0][1]:04X}..=0x{g[-1][1]:04X}")

        ethertype_pat = " | ".join(ethertype_pats)
        lines.append(f'            {ethertype_pat} => r#"{value}"#,\n')
    lines.sort()
    return "".join(lines)


def ieee(inp, out):
    reader = csv.DictReader(inp, quotechar='"', doublequote=True)
    organizations = defaultdict(list)
    organization_addresses = defaultdict(list)
    protocols = defaultdict(list)

    for row in reader:
        ethertype = int(
            re.search(r"\b[0-9A-Fa-f]{4}\b", row["Assignment"]).group(0), 16)
        organizations[row["Organization Name"]].append(ethertype)
        protocols[row["Protocol"]].append(ethertype)
        organization_addresses[row["Organization Address"]].append(ethertype)

    out.write(
        "#![allow(unreachable_patterns)]\n"
        "\n"
        "use crate::EtherType;\n"
        "\n"
        "impl EtherType {\n"
        "    /// A brief textual description of the [`EtherType`] sourced from the [IEEE Registration Authority](https://standards.ieee.org/develop/regauth).\n"
        "    pub const fn ieee_description(self) -> Option<&'static str> {\n"
        "        Some(match self.0 {\n" + format_patterns(protocols) +
        "            _ => return None,\n"
        "        })\n"
        "    }\n"
        "\n"
        "    /// The organization that registered the [`EtherType`] sourced from the [IEEE Registration Authority](https://standards.ieee.org/develop/regauth).\n"
        "    pub const fn ieee_organization(self) -> Option<&'static str> {\n"
        "        Some(match self.0 {\n" + format_patterns(organizations) +
        "            _ => return None,\n"
        "        })\n"
        "    }\n"
        "\n"
        "    /// The address of the organization that registered the [`EtherType`] sourced from the [IEEE Registration Authority](https://standards.ieee.org/develop/regauth).\n"
        "    pub const fn ieee_organization_address(self) -> Option<&'static str> {\n"
        "        Some(match self.0 {\n" +
        format_patterns(organization_addresses) +
        "            _ => return None,\n"
        "        })\n"
        "    }\n"
        "}\n")


def main():
    formats = (
        {
            "url":
            "https://www.iana.org/assignments/ieee-802-numbers/ieee-802-numbers-1.csv",
            "out": "iana.rs",
            "func": iana,
        },
        {
            "url": "https://standards-oui.ieee.org/ethertype/eth.csv",
            "out": "ieee.rs",
            "func": ieee,
        },
    )

    # The IEEE website is picky about which requests it lets through
    headers = {
        'User-Agent':
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36'
    }

    for fmt in formats:
        r = requests.get(fmt["url"], headers=headers)
        r.raise_for_status()
        path = f"src/{fmt['out']}"
        with open(path, "w") as out:
            fmt["func"](io.StringIO(r.text, newline=''), out)
        subprocess.run(["rustfmt", "--edition", "2021", path])


if __name__ == "__main__":
    main()
