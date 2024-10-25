import csv
import io
import re

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


def ieee(inp, out):
    reader = csv.DictReader(inp, quotechar='"', doublequote=True)
    assignments = []
    organization_names = []
    protocols = []
    for row in reader:
        assignments.append(
            re.search(r"\b[0-9A-Fa-f]{4}\b", row["Assignment"]).group(0))
        organization_names.append(row["Organization Name"])
        protocols.append(row["Protocol"])

    out.write(
        "#![allow(unreachable_patterns)]\n"
        "\n"
        "use crate::EtherType;\n"
        "\n"
        "impl EtherType {\n"
        "    /// A brief textual description of the [`EtherType`] sourced from the [IEEE Registration Authority](http://standards.ieee.org/develop/regauth).\n"
        "    pub const fn ieee_description(self) -> Option<&'static str> {\n"
        "        Some(match self.0 {\n" + ''.join([
            f'            0x{ethertype} => r#"{description}"#,\n'
            for (ethertype, description) in zip(assignments, protocols)
        ]) + "            _ => return None,\n"
        "        })\n"
        "    }\n"
        "\n"
        "    /// The organization that registered the [`EtherType`] sourced from the [IEEE Registration Authority](http://standards.ieee.org/develop/regauth).\n"
        "    pub const fn ieee_organization(self) -> Option<&'static str> {\n"
        "        Some(match self.0 {\n" + ''.join([
            f'            0x{ethertype} => r#"{organization}"#,\n'
            for (ethertype,
                 organization) in zip(assignments, organization_names)
        ]) + "            _ => return None,\n"
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

    for fmt in formats:
        r = requests.get(fmt["url"])
        with open(f"src/{fmt['out']}", "w") as out:
            fmt["func"](io.StringIO(r.text, newline=''), out)


if __name__ == "__main__":
    main()
