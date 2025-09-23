use crate::EtherType;

/// Common EtherTypes
///
/// The EtherTypes are taken from the following sources:
/// - [rfc7042 B.1](https://datatracker.ietf.org/doc/html/rfc7042#autoid-40)
/// - [rfc7042 B.2](https://datatracker.ietf.org/doc/html/rfc7042#autoid-41)
/// - [rfc1701 Current List of Protocol Types](https://www.rfc-editor.org/rfc/rfc1701)
/// - <https://en.wikipedia.org/wiki/EtherType#Values> (accessed 2025-09-23)
impl EtherType {
    // rfc7042 B.1
    /// Internet Protocol Version 4 (IPv4)
    pub const IPV4: Self = Self(0x0800);
    /// Address Resolution Protocol (ARP)
    pub const ARP: Self = Self(0x0806);
    /// Frame Relay ARP
    pub const FR_ARP: Self = Self(0x0808);
    /// TRILL
    pub const TRILL: Self = Self(0x22F3);
    /// L2-IS-IS
    pub const L2ISIS: Self = Self(0x22F4);
    /// Reverse Address Resolution Protocol (RARP)
    pub const RARP: Self = Self(0x8035);
    /// Internet Protocol Version 6 (IPv6)
    pub const IPV6: Self = Self(0x86DD);
    /// Point-to-Point Protocol (PPP)
    pub const PPP: Self = Self(0x880B);
    /// General Switch Management Protocol (GSMP)
    pub const GSMP: Self = Self(0x880C);
    /// MPLS
    pub const MPLS: Self = Self(0x8847);
    /// MPLS with upstream-assigned label
    pub const MPLS_MC: Self = Self(0x8848);
    /// Multicast Channel Allocation Protocol (MCAP)
    pub const MCAP: Self = Self(0x8861);
    /// PPP over Ethernet (PPPoE) Discovery Stage
    pub const PPPOE_D: Self = Self(0x8863);
    /// PPP over Ethernet (PPPoE) Session Stage
    pub const PPPOE_S: Self = Self(0x8864);
    /// TRILL Fine Grained Labeling (FGL)
    pub const TRILL_FGL: Self = Self(0x893B);
    /// TRILL RBridge Channel
    pub const TRILL_RBC: Self = Self(0x8946);

    // rfc7042 B.2
    /// IEEE Std 802.1Q   - Customer VLAN Tag Type (C-Tag, formerly called the Q-Tag) (initially Wellfleet)
    pub const C_TAG: Self = Self(0x8100);
    /// IEEE Std 802.3    - Ethernet Passive Optical Network (EPON)
    pub const EPON: Self = Self(0x8808);
    /// IEEE Std 802.1X   - Port-based network access control
    pub const PAE: Self = Self(0x888E);
    /// IEEE Std 802.1Q   - Service VLAN tag identifier (S-Tag)
    pub const S_TAG: Self = Self(0x88A8);
    /// IEEE Std 802      - Local Experimental Ethertype
    pub const IEEE802_EXPERIMENTAL1: Self = Self(0x88B5);
    /// IEEE Std 802      - Local Experimental Ethertype
    pub const IEEE802_EXPERIMENTAL2: Self = Self(0x88B6);
    /// IEEE Std 802      - OUI Extended Ethertype
    pub const OUI_EXTENDED: Self = Self(0x88B7);
    /// IEEE Std 802.11   - Pre-Authentication (802.11i)
    pub const PREAUTH: Self = Self(0x88C7);
    /// IEEE Std 802.1AB  - Link Layer Discovery Protocol (LLDP)
    pub const LLDP: Self = Self(0x88CC);
    /// IEEE Std 802.1AE  - Media Access Control Security
    pub const MACSEC: Self = Self(0x88E5);
    /// IEEE Std 802.1Q   - Multiple VLAN Registration Protocol (MVRP)
    pub const MVRP: Self = Self(0x88F5);
    /// IEEE Std 802.1Q   - Multiple Multicast Registration Protocol (MMRP)
    pub const MMRP: Self = Self(0x88F6);
    /// IEEE Std 802.11   - Fast Roaming Remote Request (802.11r)
    pub const FRRR: Self = Self(0x890D);
    /// IEEE Std 802.21   - Media Independent Handover Protocol
    pub const MIH: Self = Self(0x8917);
    /// IEEE Std 802.1Qbe - Multiple I-SID Registration Protocol
    pub const MIR: Self = Self(0x8929);
    /// IEEE Std 802.1Qbg - ECP Protocol (also used in 802.1BR)
    pub const ECP: Self = Self(0x8940);

    // rfc1701 Current List of Protocol Types
    /// SNA
    pub const SNA: Self = Self(0x0004);
    /// OSI network layer
    pub const OSI_NL: Self = Self(0x00FE);
    /// PUP
    pub const PUP: Self = Self(0x0200);
    /// XNS
    pub const XNS: Self = Self(0x0600);
    /// Chaos
    pub const CHAOS: Self = Self(0x0804);
    /// VINES
    pub const VINES: Self = Self(0x0BAD);
    /// VINES Echo
    pub const VINES_ECHO: Self = Self(0x0BAE);
    /// VINES Loopback
    pub const VINES_LOOPBACK: Self = Self(0x0BAF);
    /// DECnet (Phase IV)
    pub const DECNET: Self = Self(0x6003);
    /// Transparent Ethernet Bridging
    pub const TEB: Self = Self(0x6558);
    /// Raw Frame Relay
    pub const RAW_FR: Self = Self(0x6559);
    /// Apollo Domain
    pub const APOLLO_DOMAIN: Self = Self(0x8019);
    /// Ethertalk (Appletalk)
    pub const APPLETALK: Self = Self(0x809B);
    /// Novell IPX
    pub const IPX: Self = Self(0x8137);
    /// RFC 1144 TCP/IP compression
    pub const RFC1144: Self = Self(0x876B);
    /// IP Autonomous Systems
    pub const IP_AUTONOMOUS_SYSTEMS: Self = Self(0x876C);
    /// Secure Data
    pub const SECURE_DATA: Self = Self(0x876D);

    // https://en.wikipedia.org/wiki/EtherType#Values
    /// Wake-on-LAN
    pub const WOL: Self = Self(0x0842);
    /// Stream Reservation Protocol
    pub const MSRP: Self = Self(0x22EA);
    /// Audio Video Transport Protocol (AVTP)
    pub const AVTP: Self = Self(0x22F0);
    /// DEC MOP RC
    pub const DEC_MOP_RC: Self = Self(0x6002);
    /// DEC LAT length and 1 byte padding
    pub const LAT: Self = Self(0x6004);
    /// AppleTalk Address Resolution Protocol (AARP)
    pub const AARP: Self = Self(0x80F3);
    /// Simple Loop Prevention Protocol (SLPP)
    pub const SLPP: Self = Self(0x8102);
    /// Virtual Link Aggregation Control Protocol (VLACP)
    pub const VLACP: Self = Self(0x8103);
    /// QNX Qnet
    pub const QNX_QNET: Self = Self(0x8204);
    /// Ethernet Slow Protocols such as the Link Aggregation Control Protocol (LACP)
    pub const SLOW: Self = Self(0x8809);
    /// CobraNet
    pub const COBRANET: Self = Self(0x8819);
    /// HomePlug 1.0 MME
    pub const HOMEPLUG: Self = Self(0x887B);
    /// PROFINET Protocol
    pub const PROFINET: Self = Self(0x8892);
    /// HyperSCSI (SCSI over Ethernet)
    pub const HYPERSCSI: Self = Self(0x889A);
    /// ATA over Ethernet
    pub const ATAOE: Self = Self(0x88A2);
    /// EtherCAT Protocol
    pub const ETHERCAT: Self = Self(0x88A4);
    /// Ethernet Powerlink
    pub const POWERLINK: Self = Self(0x88AB);
    /// GOOSE (Generic Object Oriented Substation event)
    pub const GOOSE: Self = Self(0x88B8);
    /// GSE (Generic Substation Events) Management Services
    pub const GSE: Self = Self(0x88B9);
    /// SV (Sampled Value Transmission)
    pub const SV: Self = Self(0x88BA);
    /// MikroTik RoMON (unofficial)
    pub const ROMON: Self = Self(0x88BF);
    /// SERCOS III
    pub const SERCOS: Self = Self(0x88CD);
    /// HomePlug Green PHY
    pub const HOMEPLUG_GREEN: Self = Self(0x88E1);
    /// Media Redundancy Protocol (IEC62439-2)
    pub const MRP: Self = Self(0x88E3);
    /// Provider Backbone Bridges (PBB) (IEEE 802.1ah)
    pub const PBB: Self = Self(0x88E7);
    /// Precision Time Protocol (PTP) over IEEE 802.3 Ethernet
    pub const PTP: Self = Self(0x88F7);
    /// NC-SI
    pub const NCSI: Self = Self(0x88F8);
    /// Parallel Redundancy Protocol (PRP)
    pub const PRP: Self = Self(0x88FB);
    /// IEEE 802.1ag Connectivity Fault Management (CFM) Protocol / ITU-T Recommendation Y.1731 (OAM)
    pub const CFM: Self = Self(0x8902);
    /// Fibre Channel over Ethernet (FCoE)
    pub const FCOE: Self = Self(0x8906);
    /// FCoE Initialization Protocol
    pub const FCOE_IP: Self = Self(0x8914);
    /// RDMA over Converged Ethernet (RoCE)
    pub const ROCE: Self = Self(0x8915);
    /// TTEthernet Protocol Control Frame (TTE)
    pub const TTE: Self = Self(0x891D);
    /// 1905.1 IEEE Protocol
    pub const IEEE1905_1: Self = Self(0x893a);
    /// High-availability Seamless Redundancy (HSR)
    pub const HSR: Self = Self(0x892F);
    /// Ethernet Configuration Testing Protocol
    pub const LOOPBACK: Self = Self(0x9000);
    /// Redundancy Tag (IEEE 802.1CB Frame Replication and Elimination for Reliability)
    pub const REDUNDANCY_TAG: Self = Self(0xF1C1);
}
