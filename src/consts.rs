//! Common EtherTypes
//!
//! The ether types are taken from the following sources:
//! - [rfc7042 B.1](https://datatracker.ietf.org/doc/html/rfc7042#autoid-40)
//! - [rfc7042 B.2](https://datatracker.ietf.org/doc/html/rfc7042#autoid-41)
//! - [rfc1701 Current List of Protocol Types](https://www.rfc-editor.org/rfc/rfc1701)
//! - <https://en.wikipedia.org/wiki/EtherType#Values> (accessed 2024-10-25)

use crate::EtherType;

// rfc7042 B.1
/// Internet Protocol Version 4 (IPv4)
pub const IPV4: EtherType = EtherType(0x0800);
/// Address Resolution Protocol (ARP)
pub const ARP: EtherType = EtherType(0x0806);
/// Frame Relay ARP
pub const FR_ARP: EtherType = EtherType(0x0808);
/// TRILL
pub const TRILL: EtherType = EtherType(0x22F3);
/// L2-IS-IS
pub const L2ISIS: EtherType = EtherType(0x22F4);
/// Reverse Address Resolution Protocol (RARP)
pub const RARP: EtherType = EtherType(0x8035);
/// Internet Protocol Version 6 (IPv6)
pub const IPV6: EtherType = EtherType(0x86DD);
/// Point-to-Point Protocol (PPP)
pub const PPP: EtherType = EtherType(0x880B);
/// General Switch Management Protocol (GSMP)
pub const GSMP: EtherType = EtherType(0x880C);
/// MPLS
pub const MPLS: EtherType = EtherType(0x8847);
/// MPLS with upstream-assigned label
pub const MPLS_MC: EtherType = EtherType(0x8848);
/// Multicast Channel Allocation Protocol (MCAP)
pub const MCAP: EtherType = EtherType(0x8861);
/// PPP over Ethernet (PPPoE) Discovery Stage
pub const PPPOE_D: EtherType = EtherType(0x8863);
/// PPP over Ethernet (PPPoE) Session Stage
pub const PPPOE_S: EtherType = EtherType(0x8864);
/// TRILL Fine Grained Labeling (FGL)
pub const TRILL_FGL: EtherType = EtherType(0x893B);
/// TRILL RBridge Channel
pub const TRILL_RBC: EtherType = EtherType(0x8946);

// rfc7042 B.2
/// IEEE Std 802.1Q   - Customer VLAN Tag Type (C-Tag, formerly called the Q-Tag) (initially Wellfleet)
pub const C_TAG: EtherType = EtherType(0x8100);
/// IEEE Std 802.3    - Ethernet Passive Optical Network (EPON)
pub const EPON: EtherType = EtherType(0x8808);
/// IEEE Std 802.1X   - Port-based network access control
pub const PAE: EtherType = EtherType(0x888E);
/// IEEE Std 802.1Q   - Service VLAN tag identifier (S-Tag)
pub const S_TAG: EtherType = EtherType(0x88A8);
/// IEEE Std 802      - Local Experimental Ethertype
pub const IEEE802_EXPERIMENTAL1: EtherType = EtherType(0x88B5);
/// IEEE Std 802      - Local Experimental Ethertype
pub const IEEE802_EXPERIMENTAL2: EtherType = EtherType(0x88B6);
/// IEEE Std 802      - OUI Extended Ethertype
pub const OUI_EXTENDED: EtherType = EtherType(0x88B7);
/// IEEE Std 802.11   - Pre-Authentication (802.11i)
pub const PREAUTH: EtherType = EtherType(0x88C7);
/// IEEE Std 802.1AB  - Link Layer Discovery Protocol (LLDP)
pub const LLDP: EtherType = EtherType(0x88CC);
/// IEEE Std 802.1AE  - Media Access Control Security
pub const MACSEC: EtherType = EtherType(0x88E5);
/// IEEE Std 802.1Q   - Multiple VLAN Registration Protocol (MVRP)
pub const MVRP: EtherType = EtherType(0x88F5);
/// IEEE Std 802.1Q   - Multiple Multicast Registration Protocol (MMRP)
pub const MMRP: EtherType = EtherType(0x88F6);
/// IEEE Std 802.11   - Fast Roaming Remote Request (802.11r)
pub const FRRR: EtherType = EtherType(0x890D);
/// IEEE Std 802.21   - Media Independent Handover Protocol
pub const MIH: EtherType = EtherType(0x8917);
/// IEEE Std 802.1Qbe - Multiple I-SID Registration Protocol
pub const MIR: EtherType = EtherType(0x8929);
/// IEEE Std 802.1Qbg - ECP Protocol (also used in 802.1BR)
pub const ECP: EtherType = EtherType(0x8940);

// rfc1701 Current List of Protocol Types
/// SNA
pub const SNA: EtherType = EtherType(0x0004);
/// OSI network layer
pub const OSI_NL: EtherType = EtherType(0x00FE);
/// PUP
pub const PUP: EtherType = EtherType(0x0200);
/// XNS
pub const XNS: EtherType = EtherType(0x0600);
/// Chaos
pub const CHAOS: EtherType = EtherType(0x0804);
/// VINES
pub const VINES: EtherType = EtherType(0x0BAD);
/// VINES Echo
pub const VINES_ECHO: EtherType = EtherType(0x0BAE);
/// VINES Loopback
pub const VINES_LOOPBACK: EtherType = EtherType(0x0BAF);
/// DECnet (Phase IV)
pub const DECNET: EtherType = EtherType(0x6003);
/// Transparent Ethernet Bridging
pub const TEB: EtherType = EtherType(0x6558);
/// Raw Frame Relay
pub const RAW_FR: EtherType = EtherType(0x6559);
/// Apollo Domain
pub const APOLLO_DOMAIN: EtherType = EtherType(0x8019);
/// Ethertalk (Appletalk)
pub const APPLETALK: EtherType = EtherType(0x809B);
/// Novell IPX
pub const IPX: EtherType = EtherType(0x8137);
/// RFC 1144 TCP/IP compression
pub const RFC1144: EtherType = EtherType(0x876B);
/// IP Autonomous Systems
pub const IP_AUTONOMOUS_SYSTEMS: EtherType = EtherType(0x876C);
/// Secure Data
pub const SECURE_DATA: EtherType = EtherType(0x876D);

// https://en.wikipedia.org/wiki/EtherType#Values
/// Wake-on-LAN
pub const WOL: EtherType = EtherType(0x0842);
/// Cisco Discovery Protocol
pub const CDP: EtherType = EtherType(0x2000);
/// Stream Reservation Protocol
pub const MSRP: EtherType = EtherType(0x22EA);
/// Audio Video Transport Protocol (AVTP)
pub const AVTP: EtherType = EtherType(0x22F0);
/// DEC MOP RC
pub const DEC_MOP_RC: EtherType = EtherType(0x6002);
/// DEC LAT length and 1 byte padding
pub const LAT: EtherType = EtherType(0x6004);
/// AppleTalk Address Resolution Protocol (AARP)
pub const AARP: EtherType = EtherType(0x80F3);
/// Simple Loop Prevention Protocol (SLPP)
pub const SLPP: EtherType = EtherType(0x8102);
/// Virtual Link Aggregation Control Protocol (VLACP)
pub const VLACP: EtherType = EtherType(0x8103);
/// QNX Qnet
pub const QNX_QNET: EtherType = EtherType(0x8204);
/// Ethernet Slow Protocols such as the Link Aggregation Control Protocol (LACP)
pub const SLOW: EtherType = EtherType(0x8809);
/// CobraNet
pub const COBRANET: EtherType = EtherType(0x8819);
/// HomePlug 1.0 MME
pub const HOMEPLUG: EtherType = EtherType(0x887B);
/// PROFINET Protocol
pub const PROFINET: EtherType = EtherType(0x8892);
/// HyperSCSI (SCSI over Ethernet)
pub const HYPERSCSI: EtherType = EtherType(0x889A);
/// ATA over Ethernet
pub const ATAOE: EtherType = EtherType(0x88A2);
/// EtherCAT Protocol
pub const ETHERCAT: EtherType = EtherType(0x88A4);
/// Ethernet Powerlink
pub const POWERLINK: EtherType = EtherType(0x88AB);
/// GOOSE (Generic Object Oriented Substation event)
pub const GOOSE: EtherType = EtherType(0x88B8);
/// GSE (Generic Substation Events) Management Services
pub const GSE: EtherType = EtherType(0x88B9);
/// SV (Sampled Value Transmission)
pub const SV: EtherType = EtherType(0x88BA);
/// MikroTik RoMON (unofficial)
pub const ROMON: EtherType = EtherType(0x88BF);
/// SERCOS III
pub const SERCOS: EtherType = EtherType(0x88CD);
/// HomePlug Green PHY
pub const HOMEPLUG_GREEN: EtherType = EtherType(0x88E1);
/// Media Redundancy Protocol (IEC62439-2)
pub const MRP: EtherType = EtherType(0x88E3);
/// Provider Backbone Bridges (PBB) (IEEE 802.1ah)
pub const PBB: EtherType = EtherType(0x88E7);
/// Precision Time Protocol (PTP) over IEEE 802.3 Ethernet
pub const PTP: EtherType = EtherType(0x88F7);
/// NC-SI
pub const NCSI: EtherType = EtherType(0x88F8);
/// Parallel Redundancy Protocol (PRP)
pub const PRP: EtherType = EtherType(0x88FB);
/// IEEE 802.1ag Connectivity Fault Management (CFM) Protocol / ITU-T Recommendation Y.1731 (OAM)
pub const CFM: EtherType = EtherType(0x8902);
/// Fibre Channel over Ethernet (FCoE)
pub const FCOE: EtherType = EtherType(0x8906);
/// FCoE Initialization Protocol
pub const FCOE_IP: EtherType = EtherType(0x8914);
/// RDMA over Converged Ethernet (RoCE)
pub const ROCE: EtherType = EtherType(0x8915);
/// TTEthernet Protocol Control Frame (TTE)
pub const TTE: EtherType = EtherType(0x891D);
/// 1905.1 IEEE Protocol
pub const IEEE1905_1: EtherType = EtherType(0x893a);
/// High-availability Seamless Redundancy (HSR)
pub const HSR: EtherType = EtherType(0x892F);
/// Ethernet Configuration Testing Protocol
pub const LOOPBACK: EtherType = EtherType(0x9000);
/// Redundancy Tag (IEEE 802.1CB Frame Replication and Elimination for Reliability)
pub const REDUNDANCY_TAG: EtherType = EtherType(0xF1C1);
