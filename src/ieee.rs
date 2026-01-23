#![allow(unreachable_patterns)]

use crate::EtherType;

impl EtherType {
    /// A brief textual description of the [`EtherType`] sourced from the [IEEE Registration Authority](https://standards.ieee.org/develop/regauth).
    pub const fn ieee_description(self) -> Option<&'static str> {
        Some(match self.0 {
            0x0101..=0x0201 | 0x0400 | 0x0500 => r#"Invalid as an EtherType value since 1983"#,
            0x0600
            | 0x0660..=0x0661
            | 0x0801..=0x0805
            | 0x0807..=0x0808
            | 0x081C
            | 0x0844
            | 0x0884..=0x089A
            | 0x0900
            | 0x0A00..=0x0A01
            | 0x0B00..=0x0B07
            | 0x0BAD..=0x0BAF
            | 0x1000..=0x10FF
            | 0x1600
            | 0x2000..=0x207F
            | 0x22E5
            | 0x22EB
            | 0x3181
            | 0x3C10..=0x3C20
            | 0x4242
            | 0x4400..=0x4409
            | 0x454C
            | 0x5000..=0x5009
            | 0x5208
            | 0x6000..=0x6009
            | 0x6010..=0x6014
            | 0x6558..=0x6559
            | 0x7000..=0x7004
            | 0x7010..=0x7019
            | 0x7020..=0x7029
            | 0x7030..=0x7036
            | 0x7677
            | 0x8001..=0x8006
            | 0x8008..=0x8083
            | 0x809C..=0x80E3
            | 0x80F1..=0x80F7
            | 0x80FA..=0x8136
            | 0x8139..=0x8144
            | 0x8148..=0x818D
            | 0x8191..=0x821B
            | 0x8221..=0x86DC
            | 0x86DE..=0x880A
            | 0x880C..=0x880E
            | 0x8817..=0x881F
            | 0x8821..=0x8831
            | 0x8833..=0x8846
            | 0x8849..=0x885A
            | 0x885D..=0x8868
            | 0x886A
            | 0x886C
            | 0x886E..=0x886F
            | 0x8871..=0x8880
            | 0x8882..=0x888A
            | 0x888C
            | 0x888F..=0x8894
            | 0x8897..=0x8899
            | 0x889C..=0x889D
            | 0x889F..=0x88A1
            | 0x88A3
            | 0x88A5..=0x88A7
            | 0x88A9..=0x88AA
            | 0x88AC
            | 0x88AE
            | 0x88B0..=0x88B3
            | 0x88BB..=0x88BE
            | 0x88C0..=0x88C3
            | 0x88C5
            | 0x88DA
            | 0x8943
            | 0x8945
            | 0x8A96..=0x8A97
            | 0x8BC2
            | 0x8F00..=0x9003
            | 0x9040..=0x905F
            | 0xA580
            | 0xBC19
            | 0xC020..=0xC02F
            | 0xC220..=0xC22F
            | 0xFEA0..=0xFEAF
            | 0xFF00..=0xFF0F
            | 0xFFFF => r#"Protocol unavailable."#,
            0x0800 => {
                r#"IPv4 Internet Protocol Version Hornig, C., "A Standard for the Transmission of IP Datagrams over Ethernet Networks," RFC-Internet Society, Apr. 1984.                                 http://www.ietf.org/rfc/rfc894.txt"#
            }
            0x0806 => r#"Address Resolution Protocol - A. R. P."#,
            0x0842 => r#"Wake-on-LAN (WoL) as described in IEEE Std. 802"#,
            0x0A00 => r#"PARC Universal Packet (PUP)"#,
            0x0A01 => r#"PUP Address translation"#,
            0x0B01 => r#"Real Time Internet Protocol scheme 2"#,
            0x0B02 => r#"IEEE 802.3 compatible Sprite RPC"#,
            0x0B03 => r#"IETF Parameter Negotiation"#,
            0x0B04..=0x0B07 => r#"Berkeley Reserved"#,
            0x22DF => {
                r#"This Ethertype characterize the followings.
(1) Token passing control is executed on the CSMA/CD at MAC sub-layer.
 This eliminates collision occurence of the CSMA/CD to acquire deterministic characteristics.
(2) The output signal voltage at physical layer is amplified to between 8 and 16Vp-p to raise noise immunity in the electromagnetic environment inside train car."#
            }
            0x22E0 => {
                r#"NETCORE TNDUSTRAIL CO.LTD

www.netcoretec.com"#
            }
            0x22E1 => {
                r#"Sandvine does not currently wish to disclose it's protocols, but may desire to do so in the future.
"#
            }
            0x22E2 => r#"MAC Status Protocol (MSP) as defined in IEEE Std 802.1Q"#,
            0x22E3 => {
                r#"The protocol has not been published yet; however, it will be part of the ITU-T G.9960 specification, whose PHY has been consented by ITU-T. The Ethertype will be used in messages to signify that the message/primitive is specific to G.hn, the fields are defined as per IEEE 802.3. The message will have the form:
DA (6 octets)|SA (6 octets)|Ethertype/TAGs (2-x octets)|MAC client LEN (2 octets)|Data (application dependent)|FCS (4 octets)"#
            }
            0x22E4 => r#"No protocol available."#,
            0x22E6 => r#"http://expether.org"#,
            0x22E7 => r#"Congestion Notification Message (CNM) as defined in IEEE Std 802.1Q"#,
            0x22E8 => {
                r#"Ethernet bonding (inverse multiplexing) protocol. Ethernet bonding enables use of multiple low-datarate physical links to provide a high-datarate logical link, without identifying flows (as in LAG) or fragmenting frames (as in PME aggregation).
Ethernet bonding performs explicit differential delay compensation and explicit link bandwidth allocation.
Ethernet bonding can exploit links with different data-rates, and does not fragment packets. 

The Ethernet bonding header precedes the payload Ethertype.
For VLAN tagged frames it appears after the VLAN tag header.
The entire Ethernet bonding header (including Ethertype) is eight bytes in length. Following the Ethertype, there is a 2-octet &quot;sequence number&quot; field, a 1-octet &quot;version&quot; field that must be set to zero, a 1-octet &quot;reserved&quot; field for future use,
a 1-octet &quot;flags&quot; field (presently used to identify OAM messages), and a 1-octet &quot;group identifier and intermediate node traversal&quot; field.
"#
            }
            0x22E9 => r#"Congestion Notification Tag (CN-TAG) as defined in IEEE Std 802.1Q"#,
            0x22EA => r#"Stream Reservation Protocol (SRP) defined in IEEE Std 802.1Q"#,
            0x22EC => {
                r#"Protocol : Power-Line Communication Conformance Testing Control Protocol Based on Ethernet Protocol"#
            }
            0x22ED => {
                r#"There are two protocols which require two Ethernet Type.
  1. Transport VLAN:  Protocol used for traffic engineering and scalability extension.
  2.Service Provision: Protocols to support various clients.
"#
            }
            0x22EE => {
                r#"   In transport network, there are many clients such as PPP, ATM, FC, etc. It is necessary to have an Ethernet type code which is followed by a 16-bit Subtype filed. The subtype field identify the client with specific requirement such as QoS. "#
            }
            0x22EF => r#"This Ethertype will be used for FSA signalling - Q.Flowstatesig . "#,
            0x22F0 => {
                r#"IEEE Std. 1722-2016 Transport Protocol for Time-Sensitive Applications in Bridged Local Area Networks"#
            }
            0x22F1 => {
                r#"ROHC (Robust Header Compression) is an IP header compression protocol specified in IETF RFC 3095 &quot;RObust Header Compression (ROHC): Framework and four profiles: RTP, UDP, ESP, and uncompressed&quot;. The specification is available at http://www.ietf.org/rfc/rfc3095.txt."#
            }
            0x22F2 => {
                r#"HCfB (Header Compression for Broadcasting) is an IP header compression protocol for broadcasting channels, which is specified in ARIB STD B-32 Part 3 or Recommendation ITU-R BT. [MUXVLP] "Multiplexing scheme for variable-length packets in digital multimedia broadcasting systems".
ARIB standard is available at http://www.arib.or.jp/english/html/overview/doc/2-STD-B32v2_2.pdf.
Recommendation ITU-R BT.[MUXVLP] is available at  http://www.itu.int/md/R07-SG06-C-0200/en.
"#
            }
            0x22F3 => {
                r#"TRILL combine the advantages of bridges and routers and is the application of link state routing to the VLAN aware customer bridging problem. The TRILL protocol is described in the base protocol document at http://tools.ietf.org/id/draft-ietf-trill-rbridge-protocol-15.txt
The final document can be found here: http://www.ietf.org/rfc/rfc6325.txt"#
            }
            0x22F4 => {
                r#"IS-IS (Intermediate System to Intermediate System) is a link state routing protocol described in ISO/IEC 10589:2002 and IETF RFC 1195. For an example of Layer 2 use, see http://tools.ietf.org/id/draft-ietf-trill-rbridge-protocol-15.txt
The final document can be found here: http://www.ietf.org/rfc/rfc6325.txt"#
            }
            0x8086 => {
                r#"8086    Proprietary Cluster Communication Protocol
        Intel Corporation   
        2200 Mission College Blvd.
	Santa Clara, CA 95052-8119
	UNITED STATES
"#
            }
            0x809B => r#"Appletalk (Ethertalk)"#,
            0x8100 => r#"Customer VLAN Tag (C-TAG) as defined in IEEE Std 802.1Q"#,
            0x8137..=0x8138 => r#"Internetwork Packet Exchange (IPX)"#,
            0x8145..=0x8147 => {
                r#"EtherType used by the Amoeba Distributed Operating System protocols"#
            }
            0x86DD => r#"Internet Protocol Version 6 (IPV6)"#,
            0x880B => r#"PPP - IETF RFC 2637"#,
            0x880F..=0x8812 => {
                r#"Hypercom Corporation uses the EtherType fields for their proprietary Integrated Enterprise Network(IEN) LAN-to-LAN protocols."#
            }
            0x8813 => {
                r#"This is a FlashROM Loader Protocol (FLDP) implemented on the network products developed by silex technology, Inc.(Former company name: Japan Computer Industry Inc.)
The FLDP has a search list and the function of writing MAC address,
program download, and self-test etc.  
The detail of the specification is not disclosed to the public.
http://www.silex.jp"#
            }
            0x8816 => r#"Used for ZNYX Link-Layer Signaling"#,
            0x8820 => {
                r#"The EtherType Field 8820 assigned for Hitachi Cable is used for 
Hitachi Cable's proprietary plotocols such as 
 
PHDP(PHysical Discovery Protocol)
VLTP(VLan Transmission Protocol)
MMRP(Multi Master Ring Protocol)"#
            }
            0x8847..=0x8848 => {
                r#"8847: MPLS (multiprotocol label switching) label stack - unicast
 reference: RFC 3032
 URL: <ftp://ftp.rfc-editor.org/in-notes/rfc3032.txt

8848: MPLS (multiprotocol label switching) label stack - multicast
 reference: RFC 3032
 URL: <ftp://ftp.rfc-editor.org/in-notes/rfc3032.txt"#
            }
            0x885B..=0x885C => {
                r#"This EtherType is used for real-time communication between medical devices."#
            }
            0x8869 => {
                r#"BigBand Networks has developed a chassis-based platform for Internet access. Platforms can be clustered together using Ethernet interconnect.  The clustering is designed to require no configuration (i.e. is plug and play.)

To effect this BigBand Networks will be deploying its proprietary "chassis discovery" protocols over Ethernet.  These protocols require a Ethernet type."#
            }
            0x886B => r#"Nortel Networks proprietary protocol."#,
            0x886D => r#"Intel Advanced Networking Services"#,
            0x8870 => r#"LLC encapsulation as defined by IEEE Std 802.1AC."#,
            0x8881 => {
                r#"This assignment will be used to identify the byte stream protocol that is used for IP based micro-mobility bearer interfaces (A10) in cdma2000® based wireless networks."#
            }
            0x888B => {
                r#"TOSHIBA utilizes Ethernet based communications network, as a real-time control network in large number of actual manufacturing fields worldwide, which is required highly deterministic transmissions, co-existence of time-critical and non-time-critical data transmissions without prejudice to the time-critical data delivery, and  regular Ethernet based applications running in parallel to the real-time
control applications in the same network. The protocol processes the deterministic media access for prioritized data delivery as the extension of the Ethernet MAC protocol identified by the EtherType Field of 888b, and allows deterministic time-critical data broadcasting for the synchronization, the temporal and spatial data coherency of whole I/O data and the network management data on the global data storage inside and common to each node in the network. The I/O data is for control, monitor and
supervisory use of filed devices, and the network management data is used for the redundancy within the required time constraint."#
            }
            0x888D => {
                r#"The ANSI FC-BB family of standards that have been developed, or are being developed within the INCITS T11 organization specify that all FC-BBW messages carry a fixed 8 byte Logical Link Control (LLC)/Sub Network Access Protocol (SNAP) field. The FC-BB-x LLC/SNAP header specifies an OUI of hex '00000' and requires a two byte PID that specifies Fibre Channel. This is really not something that should be assigned to a single company but instead be associated with the INCITS T11 organization or ANSI."#
            }
            0x888E => {
                r#"Port Access Entity (PAE) for Ethernet Access Protocol (EAP) over LAN (EAPOL) as defined in IEEE Std 802.1X"#
            }
            0x8895 => {
                r#"This ethertype is used exclusively by networks protected by the AirFortress Security solution used in enterprise wireless networks and wireless point-to-point bridges. The AirFortress security protocol provides FIPS 140 validated data encryption above the MAC layer for all 8895 wireless or wired traffic. This protected traffic is recognized, tracked by ethertype which can then be monitored by various network management platforms. This traffic type is vendor, network, and protocol agnostic and can be implemented across a wide range of 802. compliant networking gear."#
            }
            0x8896 => {
                r#"Digigram provides professional audio over IT network solutions targeted at broadcast, public-address and other professionnal applications.

EtherSound(tm) enhances established technologies to provide easy-to-implement, high-quality audio networks. The patented EtherSound protocol provides fully deterministic, very low-latency transmission of synchronized audio channels over standard Ethernet. Up to 64 channels of 24-bit digital audio at 48 kHz, plus bi-directional status and control data, may be transported among more than 60,000 connected devices."

(http://www.digigram.com/products/by_category.htm?o=getcat&pcat_key=PCNTA&t=Networked+Audio+Devices+and+Technologies)"#
            }
            0x889A => r#"SCSI over Ethernet"#,
            0x889B => {
                r#"This EtherType Field designates the CSM_ENCAPS protocol.  It is a connectionless protocol designed to allow the assignment of Ethernet MAC addresses to devices on the Ethernet, as well as reliable data transport once MAC addresses are assigned.  The CSM_ENCAPS protocol is typically used in media gateway systems designed using Mindspeed Technologies devices.  Refer to www.mindspeed.com for further information."#
            }
            0x889E => {
                r#"Remotec Robot deterministic heartbeat and safety critical control and status protocol for hazardous duty robotic vehicles."#
            }
            0x88A2 => r#"Advanced Technology Advancement (ATA) Protocol"#,
            0x88A4 => r#"EtherCAT"#,
            0x88A8 => {
                r#"Service VLAN Tag (S-TAG) or Backbone VLAN Tag (B-TAG) as defined in IEEE Std 802.1Q"#
            }
            0x88AB => r#"ETHERNET Powerlink (EPL)"#,
            0x88AD => {
                r#"We are manufacturing a series of devices that are connected to the Ethernet requiring secure data transmission.  Because of the security reasons, the communication between the devices is done through a proprietary communication protocol instead of well known protocols such as IP.  We therefore need an Ethertype Field for the Ethernet frame to embed our proprietary communication protocol."#
            }
            0x88AF => {
                r#"Proprietary automotive control unit protocol used by UK OEM Life Racing
Ltd."#
            }
            0x88B4 => {
                r#"WAI is a new authentication protocol that will be used to access authentication in IP based networks. This protocol establishes a logic channel between a station and access equipment by using an EtherType Field to accomplish authentication. WAI can offer secure access control by mutual certificate authentication based on AS (authentication server).
WAI is more secure than the most existing authentication protocol and can protect network against attacks.
Now we want to request a new EtherType Field to distinguish our new protocol from all all existing protocol in actual network."#
            }
            0x88B5 => {
                r#"Local Experimental EtherType 1 as defined in IEEE Std 802.  This EtherType value is available for public use and for prototype and vendor-specific protocol development."#
            }
            0x88B6 => {
                r#"Local Experimental EtherType 2 as defined in IEEE Std 802.  This EtherType value is available for public use and for prototype and vendor-specific protocol development."#
            }
            0x88B7 => {
                r#"OUI Extended EtherType as defined in IEEE Std 802. This EtherType value is available for public use and for prototype and vendor-specific protocol development."#
            }
            0x88B8..=0x88BA => {
                r#"IEC 61850 is a global standard for the use in utility communication, in particular for the information exchange between IED's in a power transmission or distribution substation.

There are three types of application services that use a specific EtherType. GOOSE uses EtherType field 88b8, GSE management services uses EtherType field 88b9. These two protocols are defined in IEC 61850-8-1. SV (Sampled Value Transmission) uses EtherType field 88ba; the protocol is defined in IEC 61850-9-1 and IEC 61850-9-2."#
            }
            0x88BF => {
                r#"For encapsulation of Ethernet packets being transmitted between switch-like
devices to carry device specific information"#
            }
            0x88C4 => {
                r#"This Ethertype is used to identify a protocol used for automatic configuration of Wireless LANs.  For further information, please visit www.AutoCell.com."#
            }
            0x88C6 => r#"This product is intended for 802.11 product to product communications."#,
            0x88C7 => r#"RSNA Preauthentication as defined in IEEE Std 802.11"#,
            0x88C8 => {
                r#"In our protocol, only one field must be required. It is two byte "sub-type" field. We manage this sub-type and assign it to each application.  Acutual protocol in each application may be vary. If sub-type is allocated, its protocol and data format can be freely designed."#
            }
            0x88C9 => r#"Ethernet Protocol"#,
            0x88CA => {
                r#"TIPC  (Transparent Inter Process Communication, http://tipc.sourceforge.net/) is designed for efficient  intra cluster communication.  Main features:   1) A functional addressing scheme, providing location transparency by hiding the cluster's physical topology for the application programs. 2)Lightweight, reactive, connections:  No hidden protocol message exchange during connection setup/shutdown. Immediate connection abortion and problem report upon any kind of service failure.  3) Generic, adaptive, signalling link protocol: Retransmission, segmentation, bundling and continuity check are performed at the signaling link layer, enabling better resource usage and performance than with traditional transport layer protocols. 4)In-sequence, loss-free message delivery in both connection oriented and connectionless mode. 5) Possibility for applications to subscribe for the availability/non-availability of functional and physical addresses."#
            }
            0x88CB => {
                r#"The applying EtherType number will be used to mark EPA(Ethernet for Plant Automation) protocol,includeing application and user layers protocols,system management function block,etc,based on Ethernet or IEEE802.3 and TCP/IP protocols.
a URL for the protocol: http://www.epa.net.cn  notes:This is a chinese website,we are translating it to English.
"#
            }
            0x88CC => r#"Link Layer Discovery Protocol (LLDP) defined in IEEE Std 802.1AB"#,
            0x88CD => r#"SERCOS interface "#,
            0x88CE => {
                r#"Type Protocol:

Remote Direct Memory Access over Ethernet (RDMAoE): a protocol for low-latency, low-overhead memory-to-memory communications among hosts interconnected by Ethernet
"#
            }
            0x88CF => {
                r#"In order to manage and control the WBS not to affect the protocol between users, it is a protocol for connecting layer 2 about between "Manager" and WBS.
The "Manager" and WBS encapsulate data / management / control information transmitted on radio, and the various information by Ethernet, and transmit them to each other."#
            }
            0x88D0 => {
                r#"Nortel Proprietary Transit Link discovery protocol for WLAN mesh product."#
            }
            0x88D1 => {
                r#"Not currently published
Company Wedsite www.integralaccess.com"#
            }
            0x88D2 => {
                r#"This will be used in a revision of the Interoperability Specification (IOS) for cdma2000 Access Network Interfaces (document numbers A.S0011-B through A.S0017-B v1.0). This document already uses the Ether type 8881 to carry bearer data but a new number is needed to distinguish new signaling information that will be added in the next revision of the document. The most recently published version of the document is available at http://www.3gpp2.org/Public_html/specs/index.cfm#tsga."#
            }
            0x88D3 => {
                r#"This is a proprietary and confidential protocol for the use of real-time communications with a future Digidesign product. We respectfully request that we do not document its details here.

 

 

"#
            }
            0x88D4 => {
                r#"The RapidRing protocol will send a â€œlink repair requestâ€ Ethernet frame when the switch detects a link lost on one of its ports. This will be received by the root node of the network. The root node will enable its backup port and send a â€œlink repairedâ€ Ethernet message. When a non-root node switch detects a repaired link it will send a â€œnew linkâ€ Ethernet message."#
            }
            0x88D5 => {
                r#"Nortel Networks
"#
            }
            0x88D6 => r#"Aruba Networks L2 AES"#,
            0x88D7 => {
                r#"The purpose of TT Ethernet is to provide a seamless communication system for different types of distributed non-real-time and real-time applications of different criticality, from very simple uncritical data acquisition tasks, to multimedia systems and up to safety-critical control applications, such as fly-by-wire or drive-by wire.

The protocol details are currently under development - for details look at http://www.vmars.tuwien.ac.at
"#
            }
            0x88D8 => {
                r#"The Metro Ethernet Forum's MEF 8, "Implementation Agreement for the Emulation of PDH Circuits over Metro Ethernet Networks", provides for the emulation of TDM services belonging to the Plesiochronous Digital Hierarchy (PDH) across a Metro Ethernet Network.  Specifically it covers emulation of Nx64 kbit/s, DS1, E1, DS3 and E3 circuits.  Generically this is referred to as Circuit Emulation Services over Ethernet (CESoETH)."#
            }
            0x88D9 => {
                r#"The protocol utilizes MAC layer encapsulation for point to point, point to multipoint and broadcast communication.  The protocol is used to interconnect devices for real-time and near-real time discovery, control and eventing."#
            }
            0x88DB => {
                r#"Adtran does not currently wish to disclose it's protocols, but may desire to do so in the future."#
            }
            0x88DC => r#" (WAVE) Short Message Protocol (WSM) "#,
            0x88DD => {
                r#"AES50-2005 (publication pending); other projects in development.  All AES standards publications on-line at http://www.aes.org/publications/standards"#
            }
            0x88DE => {
                r#"Akimbi does not currently wish to disclose it's protocols, but may desire to do so in the future."#
            }
            0x88E0 => r#"Ethernet Trunks"#,
            0x88E1 => r#"HomePlug Specification AV MME"#,
            0x88E2 => {
                r#"Japan Radio Co., Ltd.
JRC Layer two protocol.
www.jrc.co.jp
"#
            }
            0x88E3 => r#"MRP (medium redundancy protocol)"#,
            0x88E4 => {
                r#"Via NeTworking Switch Management Protocol (VNTSMP) provides one such solution. Switches that support VNTSMP send and receive VNTSMP PDU with one another. A switch without a CPU module (the Client) uses VNTSMP PDU (QUERY) to encapsulate control protocol PDU, such as BPDU, to the switch equipped with a CPU module (the Server) for processing. The Server then sends the Client a response PDU (REPLY) to tell it what action needs to be taken. The Server can also send a QUERY PDU to the Client for a snapshot of all statistics counters. The Client then returns it in one or more REPLY PDUs.
"#
            }
            0x88E5 => r#"Media Access Control (MAC) Security tag as defined in IEEE Std 802.1AE"#,
            0x88E6 => r#"Nortel Ethernet OAM protocol."#,
            0x88E7 => r#"Backbone Service Instance Tag (I-TAG) as defined in IEEE Std 802.1Q"#,
            0x88E8 => r#"LVL7 Systems Proprietary Protocol"#,
            0x88E9 => {
                r#"Motorola does not currently wish to disclose this protocol but may desire to do so in the future.
"#
            }
            0x88EA => {
                r#"For the use of transferring a new security authentication and encryption protocol between two devices at the L2 layer.  The protocol will transfer 802.1x and 256 bit AES-CBC encrypted frames across wired and wireless devices by encapsulating L2 packets.  The new ether type is needed to identify and differentiate from other packets in order to apply the the proper decryption functions."#
            }
            0x88EB => r#"This is an application for a second e-type from Aruba at this time."#,
            0x88EC => {
                r#"Talari Encapsulation Protocol is used to commnuicate between Talari systems deployed over public IP networks. Using the protocol Talari systems create a virtual overlay network to provide enterprise connectivity over public IP networks"#
            }
            0x88ED => r#"Meshcom Mesh Protocol (MMP). www.meshcom.com"#,
            0x88EE => {
                r#"Ethernet Local Management Interface (E-LMI).
http://www.metroethernetforum.org/TechSpec.htm
(The document number is "MEF 16.")"#
            }
            0x88EF => {
                r#"NVIDIA System Management Control Protocol.

(No URL exists for this protocol, which is NVIDIA proprietary)"#
            }
            0x88F0 => {
                r#"IEEE P1451.0 Smart Transducer Interface for Sensors and Actuators

http://grouper.ieee.org/groups/1451/0/private/

"#
            }
            0x88F1 => {
                r#"This ethertype is used exclusively by Rajant's BreadCrumb(R) Wireless Network.  Rajant's BreadCrumb Wireless Network is a robust, rapidly deployable mesh network with a highly mobile infrastructure.  For more information please visit http://www.rajant.com"#
            }
            0x88F2 => {
                r#"WNSIA - Wireless Network for Secure Industrial Application protocol suite"#
            }
            0x88F3 => r#"protocol unavailable"#,
            0x88F4 => {
                r#"This EtherType is used for real-time communication between industrial control equipment."#
            }
            0x88F5 => {
                r#"Multiple VLAN Registration Protocol (MVRP) defined in IEEE Std 802.1Q 
"#
            }
            0x88F6 => r#"Multiple MAC Registration Protocol (MMRP) as defined in IEEE Std 802.1Q"#,
            0x88F7 => {
                r#"The EtherType field assignment is to be used in the revised IEEE 1588, Standard for a Precision Clock Synchronization Protocol for Networked Measurement and Control Systems.  

The URL for the standard activities is: http://ieee1588.nist.gov
"#
            }
            0x88F8 => {
                r#"This EtherType is used to support the protocol called NC-SI (Network
Controller - Sideband Interface) control command and response protocol
which is defined by the DMTF NC-SI Specification to be published in
early 2007."#
            }
            0x88F9 => r#"Alcatel Proprietary Protocol"#,
            0x88FA => {
                r#"VARAN (Versatile Automation Random Access Network) 
is a manufacturer independent Ethernet based real-time network protocol. 

The highlights are speed, openness, easy implementation and low costs. The Bus is seen as a 4GB memory area. The client access is realised by simple memory read/write commands.
Other Ethernet protocols like TCP/IP can be transported thru the VARAN-Bus.
The protocol is completely implemented into FPGA Hardware.
"#
            }
            0x88FB => {
                r#"The IEC 62439 PRP protocol operates by sending the normal (IP,..) Ethernet frames over two independent networks, while appending the frames (except BPDUs) with a redundancy control trailer to help the receiver in discarding the duplicates. 
The Ethertype identifies supervision frames that check the livelyness and presence or absence of nodes.
URL:"#
            }
            0x88FC => {
                r#"The protocol will use a common header multiplexing various subprotocols into a single ethertype. The format is as follow:

Version   8 bits  Version of the common header
Subtype   8 bits  Subprotocol identifier
Flag      8 bits  Dependent subprotocol flag bits
Offset    8 bits  Offset to the subprotocol PDU

The actual subprotocol PDU location is computed by adding the Offset value to the address of the Offset field itself. For example  an Offset of 0 point to the byte following the Offset field."#
            }
            0x88FD => {
                r#"The protocol consists of extra fields prefixing a standard IP packet all of which is then encapsulated by a standard Ethernet packet.

The extra fields consist of an Identifier (16-bits), length (16-bits) and status (8-bits)."#
            }
            0x88FE => {
                r#"The newly developed protocol of LS Industrial Systems is the real-time Ethernet (RTE) protocol that designed for supporting several topologies of ring, line and star based on ISO/IEC 8802-3. Especially this protocol is suitable for high reliable automation network control systems that need minimum redundancy recovery time. The redundancy recovery time means the maximum time from failure to become fully operational again in case of a single permanent failure
The company website is www.lsis.biz (Korean) or www.lgis.com (English).
"#
            }
            0x88FF => {
                r#"This Ethertype is used to identify a protocol
used for Fujitsu MAC Security and so on."#
            }
            0x8900 => r#"Red Lion Controls Inc. proprietary"#,
            0x8901 => r#"Flow Layer Internal Protocol (FLIP) for inter-unit messaging."#,
            0x8902 => {
                r#"Connectivity Fault Management (CFM) Protocol Data Unit (PDU) Encapsulation as defined in IEEE 802.1Q"#
            }
            0x8903 => r#"DCE"#,
            0x8904 => r#" BCN (Backward Congestion Notification) data frame tag"#,
            0x8905 => {
                r#"T-Tag (Timestamp Tag)
This tag carries timestamp information as part of the Ethernet frame."#
            }
            0x8906 => {
                r#"FCoE - Fibre Channel over Ethernet
"#
            }
            0x8907 => {
                r#"DRP(Distributed Redundancy Protocol) is a high availability ethernet protocol developed by Zhejiang University. This protocol guarantee short recovery time in case of either link failure or switch failure. And in this protocol, all switches are distributed and equal without master and slave. This protocol is useful for ring/tree/star topology."#
            }
            0x8908 => {
                r#"Following the EtherType are the following fields, 2 bytes each in little-endian order:
Workgroup ID, Destination LUN, Source LUN, Opcode. The combination of the first three fields along with the Ethernet address is used for classifying packets into flows. The Opcode field determines the structure of the information contained in the remaining part of the packet. Detailed information regarding the packet structure for different opcode values is not being made public at this time."#
            }
            0x8909 => r#"Cisco Metadata (CMD)"#,
            0x890A => {
                r#"Protocol WIO 
For Building Automation "#
            }
            0x890B => r#"Panduit Proprietary Protocol."#,
            0x890C => {
                r#"FibroLAN CPE Management Protocol (FCMP)
"#
            }
            0x890D => r#"Management protocol as defined in IEEE Std 802.11"#,
            0x890E | 0x8918 | 0x8923 | 0x8934 | 0x894D => r#"Protocol unavailable"#,
            0x890F => {
                r#"This protocol is especially suitable for industrial networks.
The protocol is newly developed to support real-time communications and ring topology, based on IEEE 802.3.
Adopting the redundancy ring, the protocol provides high availability."#
            }
            0x8910 => {
                r#"Encapsulated Addresses for use with the Backbone Service Instance as defined in IEEE Std 802.1Q"#
            }
            0x8911 => {
                r#"The complete description of the LINX protocol, and related documentation, may be found at:
	https://sourceforge.net/projects/linx/
This is the SourceForge web site. Look under Project/Web to direct you to the LINX homepage. Or you may go directly to the documents homepage at:
	http://linx.sourceforge.net
"#
            }
            0x8912 => r#"Ethertype used for mediaxtream Specification protocols"#,
            0x8913 => {
                r#"The ethertype is used for an ethernet security protocol used by ethernet link encryption devices manufactured by Rohde & Schwarz."#
            }
            0x8914 => r#"FIP Storage Access Protocol"#,
            0x8915 => r#"RoCE - RDMA over Converged Ethernet"#,
            0x8916 => {
                r#"PXL video data protocol. Valid frame lengths are 64 octets up to 8192 octets.
Valid first two octets of frame payload are: (00-01 thru 00-6F), (00-EE thru 00-FE),
(80-10 thru 80-6F), (80-EE thru 80-FE)."#
            }
            0x8917 => r#"Media Independent Service (MIS) protocol as defined in IEEE Std 802.21"#,
            0x8919 => {
                r#"Two protocols will be using this EtherType:

1) Xsigo Discovery Protocol (XDP). Used by servers to discover I/O Directors which are configured to provide virtual I/O devices for the servers.

2) Xsigo Session Management Protocol (XSMP). Used by servers to receive information about the individual virtual I/O devices allocated to them on the I/O Directors residing on the Ethernet network.
"#
            }
            0x891A => {
                r#"The protocol for this Ethertype will allow for software above the driver to periodically send packets. The periodicity allows for significant power savings for devices operating on small batteries. The power saving nature of this protocol requires that it operate at the Ethertype level; it does not fit well into the context of the TCP/IP protocol suite.

Our first implementation used an unique OUI for Intel; however, the driver stack from Microsoft Vista requires the use of an EtherType, not an OUI value.
"#
            }
            0x891B => r#"TAEPOL (TAEP over LANs)."#,
            0x891C => {
                r#"This protocol is used for high bandwidth, low latency transmission of video data to video wall display devices over an Ethernet network. The protocol is proprietary to Martin Professional A/S, refer to www.martin.dk for further information."#
            }
            0x891D => r#"TTEthernet  Protocol Control Fram (TTE)"#,
            0x891E => {
                r#"CipherOptics does not currently wish to disclose this protocol but may desire to do so in the future.

www.cipheroptics.com"#
            }
            0x891F => {
                r#"ITU-T Recommendation G.7041 Generic Framing Procedure (GFP)

http://www.itu.int/net/home/index.aspx
"#
            }
            0x8920 => {
                r#"Proprietary transport protocol distributing Full HD video and lossless audio to networked devices.
www.XStreamHD.com"#
            }
            0x8921 => r#"Decline to disclose. Cisco Proprietary"#,
            0x8922 => {
                r#"Following the ethernet header, the packet contains a 4-byte magic number, a 2-byte length field, and a 1-byte type field. Additional data are type dependent. For a "beacon" packet, the data includes a unique host identifier, a sequence number, a source virtual port identifier, and the name of the physical adapater.

www.vmware.com"#
            }
            0x8924 => {
                r#"Vendor-specific protocol with multiple proprietary sub-protocols; mainly used for fast, local, TCP/IP-independent inter-device communication, as well as for bootloading, debugging and initial configuration of iris devices with Ethernet interface. iris does not currently wish to disclose its protocol specification, but may decide to do so in the future.
Please refer to http://www.irisgmbh.de
"#
            }
            0x8925 => r#"Fortress Mesh Protocol"#,
            0x8926 => {
                r#"VNTAG - Virtual Network endpoint TAG
This protocol allows for aggregation of multiple network endpoints onto a single physical link in a manner that lives below the wire protocols seen by operating system stacks and traditional network links.  The architecture relies on a Virtual Interface Switch that logically terminates each of these endpoints and forwards/filters as appropriate.  This is achieved by providing a means to
- identify a specific endpoint at ingress
- select a specific endpoint or set of endpoints as destinations"#
            }
            0x8927 => {
                r#"CopperLan protocol is dedicated to command-and-control in the environments of music, pro-audio, stage and related equipment, both in embedded products and computer applications.
www.copperlan.org "#
            }
            0x8928 => r#"FSPF over Ethernet"#,
            0x8929 => {
                r#"Multiple I-SID Registration
Protocol (MIRP) as defined in IEEE Std 802.1Q
"#
            }
            0x892A => {
                r#"REAC (Roland Ethernet Audio Communication) protocol.
Payload structure = Frame number (2 bytes) | Control and Audio Data  
(variable) | Frame Format Type (2 bytes)"#
            }
            0x892B => {
                r#"SEL develops mission critical hard real time systems for electric power system protection, control and
monitoring. SEL uses this EtherType to facilitate exchange of priority tagged mission critical Layer 2
messages using distributed architectures with Ethernet connectivity. For VLAN tagged frames, this
EtherType appears after the VLAN tag header. EtherType is followed by a 16 bit sub-type field managed
by SEL.
"#
            }
            0x892C => {
                r#"TLP (TePA based LAN Privacy) protocol. 

Payload structure = TLP Ethertype (2 bytes) | Version(3 bits)|IsEncrypted(1 bit)| Encrypt mode (4bits)| Control Data (variable)|Secure Data(variable) |MIC(16 bytes)

"#
            }
            0x892D => {
                r#"Payload structure = Protocol_Header (10 bytes total, where:byte#1 = sub-type, byte#4 = data_block_count);  Data_Block_Sequence (each has 24-byte block header plus variable data);
Padding (0-32 bits, variable)."#
            }
            0x892E => {
                r#"A protocol to enable USB type data and USB type commands to be sent between two devices connected by an IEEE802 link, typically an IEEE802.11 link. The protocol allows encoding of all information that would normally be present in a wired USB request, such as request_id and rcode along with payload data. Defined header structures allow correct delivery of the required fields. The informaion provided through the protocol enables USB like behaviour to be implemented between wirelessly connected devices. www.ozmodevices.com
"#
            }
            0x892F => r#"High-availability Seamless Redundancy (HSR) protocol"#,
            0x8930 => {
                r#"Layer II protocol for data center switches
"#
            }
            0x8931 => {
                r#"We will define a protocol called "IC Plus Management Protocol". It's also called "IPMP" for abbreviation. With this protocol, the network administrator can manage and monitor the network devices supporting this protocol."#
            }
            0x8932 => {
                r#"Mellanox discovery and configuration protocol.
"#
            }
            0x8933 => r#"Infinera proprietary optical transport"#,
            0x8935 => {
                r#"VDS does not currently wish to disclose this protocols, but may desire to do so in the future.
 
Company website:  http://www.vds-it.com
"#
            }
            0x8936 => {
                r#"A protocol for exchange of radio data frames between a network forwarding device and a radio device. Protocol details may be released at a later stage."#
            }
            0x8937 => {
                r#"Allows Panasas switch to send periodic status messages to Panasas blades."#
            }
            0x8938 => r#""HDBaseT Control and Management Protocol (www.hdbaset.org)""#,
            0x8939 => {
                r#"Generic Associated Channel (G-ACh) protocol encapsulation over Ethernet.
This type indicates that the Ethernet payload begins with an Associated
Channel Header (ACH) as defined in RFC 5586
(http://tools.ietf.org/html/rfc5586).  The ACH further identifies the
format of the packet that follows.  Please refer to RFC 5586 for
details.
"#
            }
            0x893A => {
                r#"The P1905.1 standard defines an abstraction layer for multiple home networking technologies. The abstraction layer provides a common data and control Service Access Point to the heterogeneous home networking technologies described in the following specifications: IEEE 1901, IEEE 802.11, IEEE 802.3 and MoCA 1.1. The standard is extendable to work with other home networking technologies. 

The abstraction layer supports dynamic interface selection for transmission of packets arriving from any interface (upper protocol layers or underlying network technologies). End-to-end Quality of Service (QoS) is supported.

Also specified are procedures, protocols and guidelines to provide a simplified user experience to add devices to the network, to set up encryption keys, to extend the network coverage, and to provide network management features to address issues related to neighbor discovery, topology discovery, path selection, QoS negotiation, and network control and management.

http://grouper.ieee.org/groups/1905/1/index.html"#
            }
            0x893B => {
                r#"This EtherType is expected to be useful in a number of applications
but this application is particularly motivated by standards use in an
extension to the IETF TRILL protocol standard. This EtherType
provide a way to supply a 12-bit extension to the VLAN ID in a
proceeding VLAN tag and space for an additional 3-bit priority value.
See the Protocol description at
http://www.pothole.com/~dee3/drafts/draft-eastlake-trill-rbridge-fine-labeling-02.txt,
especially Section 2.3."#
            }
            0x893C => {
                r#"The Coraid Ethernet Console (CEC) protocol defines and implements a bidirectional conversation over raw ethernet frames with provisions for retransmission and discovery.  The CEC protocol is integrated with a console server and CEC clients in its first implementation, providing a central management solution for Coraid's appliances.

http://sources.coraid.com/magic/man2html/8/cec"#
            }
            0x893D => {
                r#" It is a general configuration that PLC modems in local areas are connected with WAN(wide area network) by one PLC gateway for the last mile of an access network. In this network structure, the communication should be preceded with authentication of devices, checking channel information and remote control of PLC modems for stable and reliable operation.
  For reference, several ether types for the PLC network are currently operated by PLC chip manufacturers such as Mavell, Intellon, Xeline and so on. KEPCO, an utility company, plans to operate an independent ether type to achieve device-level interoperability."#
            }
            0x893E => {
                r#"EL is a connection-based, lightweight transport protocol, providing the transport layer for 9P protocol while avoiding the overhead and complexity of Internet Protocol within the same network broadcast domain. It provides retransmission of lost messages and in-sequence delivery, but has no flow control and no blind retransmission.

"#
            }
            0x893F => {
                r#"Bridge Port Extension tag (E-TAG) as defined in IEEE Std 802.1BR
"#
            }
            0x8940 => {
                r#"Edge Control Protocol (ECP) defined in IEEE Std 802.1Q for use with IEEE Std 802.1BR"#
            }
            0x8941 => {
                r#"Cambium Networks proprietary block-oriented point-to-point streaming protocol."#
            }
            0x8942 => {
                r#"Following the ethernet header, the packet contains a 4-byte magic number, a 2-byte length field, and a 1-byte type field. Additional data are type dependent.
URL: www.bigswitch.com
"#
            }
            0x8944 => {
                r#"High bandwidth, low latency video over Ethernet protocol. All frames
contain a four octet packet header. First octet contains Protocol ID,
third and fourth octet contain 11 bit packet length. Valid Protocol ID
is 0 or 1. Valid length is between 4 and 1522 octets."#
            }
            0x8946 => {
                r#"The RBridge Channel protocol is specified in
http://www.ietf.org/id/draft-ietf-trill-rbridge-channel-05.txt. Most of the
document is about communication between RBridges. Section 4 describes
the differences for transmission between and end station and an RBridge. 
"#
            }
            0x8947 => {
                r#"GeoNetworking as defined in ETSI EN 302 636-4-1.

Link to the protocol:

http://webapp.etsi.org/workprogram/Frame_WorkItemList.asp?SearchPage=TRUE&qSORT=HIGHVERSION&qINCLUDE_SUB_TB=True&butSimple=++Search++&qETSI_STANDARD_TYPE=&qETSI_NUMBER=302+636-4-1&qETSI_ALL=TRUE&qMILESTONE=&qACHIEVED_DAY=&qACHIEVED_MONTH=&qACHIEVED_YEAR=&qREPORT_TYPE=SUMMARY&optDisplay=10&qTB_ID=&includeNonActiveTB=FALSE
"#
            }
            0x8948 => r#"EoIB: Ethernet over InfiniBand Protocol"#,
            0x8949 => r#"Mellanox Remote Mirror Encapsulation"#,
            0x894A => {
                r#"http://www.endace.com/endace-ethernet.html

Endace Ethernet is used to transport Endace ERF frames across conventional ethernet networks.  Because ERF records are not a well defined standard, a dedicated ethernet assignment avoids mis-analysis.

Each frame contains a subtype field, possible sequence number, a number of encapsulated records, possible meta data and the records themselves.  For subtype 0x01 the records are in ERF (Endace Record Format).  The only defined subtype at present is 0x01."#
            }
            0x894B => r#"flow filtering tag (F-TAG) as defined in IEEE Std 802.1Q"#,
            0x894C => {
                r#"IEC 61375 TCN (Train Communication Network) is a series of international standards for the use in communication on-board of trains, in particular for the information exchange between TCMS EDs (Train Control and Monitoring System End Devices), e.g. door control, brake control units, traction control units and diagnostic units. 

The IEC61375-2-5 and IEC61375-3-4 parts cover respectively the hierarchical communication stacks relevant to:

1)	the Train Backbone Network (communication along the train)

2)	the Consist Network (communication within the vehicle or group of vehicle).
"#
            }
            0x894E => {
                r#"FSM Solutions does not currently wish to disclose it&#39;s protocols, but may desire to do the future."#
            }
            0x894F => r#"894F: NSH (Network Service Header). Reference: draft-ietf-sfc-nsh-18"#,
            0x8950 => {
                r#"The Fast Networking and Transport Protocol (FNTP) is a "stripped down" networking and transport layer protocol that is essentially a "port-mapping" protocol with no networking overhead incurred at all.  It is intended for use in situations where channel capacity is limited (necessitation a minimal overhead protocol) and speed in processing of NPDUs is important.  The details of the protocol can be found in the ISO 29281 international standard."#
            }
            0x8951 => r#"BTS internal protocol for inter-unit messaging"#,
            0x8952 => r#"Distributed Relay Control Protocol (DRCP) as defined in IEEE Std 802.1AX"#,
            0x8953 => {
                r#"Proprietary protocol
"#
            }
            0x8954 => {
                r#"The protocol is a proprietary one; however each Ethernet packet payload will consist of a header describing the proprietary packet. The aforementioned header contains a byte describing the type and a byte describing the version of the header, thus allowing for future expansion. The type and version byte have deliberately been made the first two bytes allowing the structure of the header to be changed in the future if required."#
            }
            0x8955 => {
                r#"High efficiency, low latency proprietary streaming data distribution protocol."#
            }
            0x8956 => {
                r#"Protocol_Type (Ethertype)    16 bits
Protocol Subtype                    16 bits
Protocol Version                     8 bits
"#
            }
            0x89A2 => {
                r#"Congestion Isolation Message (CIM) as defined in IEEE 802.1Qcz amendment to IEEE Std 802.1Q"#
            }
            0x8CE4 => r#"This Ethertype is used to embed additional metadata in Ethernet frames"#,
            0x909B => r#"V2V protocol invented by VisionVera.  "#,
            0x9433 => {
                r#"The protocol for this Ethertype is specified by O-RAN Alliance and is found on https://www.o-ran.org/
"#
            }
            0x96B6 => {
                r#"The industrial optical fieldbus is derived from ITU-T GPON standard and reformed for industrial control application to ensure low latency, deterministic and high reliability connection between the control device and field devices."#
            }
            0x9999 => {
                r#"ADTRAN does not currently wish to disclose it's protocols, but may desire to do so in the future."#
            }
            0x99FE => {
                r#"CMP enables the recording of the data of in-vehicle communication systems via a decentralized logging network based on Ethernet as physical layer. This allows remote monitoring and control of data captured via different interfaces from the capture modules"#
            }
            0x9A22 => {
                r#"See IETF Internet Draft draft-ietf-trill-multi-topology-06.txt which is about to be published as RFC 8377."#
            }
            0x9AC6 => r#"TTP (Tesla Transport Protocol) over Ethernet"#,
            0x9C40 => {
                r#"SafetyNET p is a deterministic real-time Ethernet for the industrial environment. Established technology from the SafetyBUS p safe bus system was also considered and refined. So SafetyNET p is an Ethernet-based network for industry, which can be used for real-time and safe communication functions.
http://www.pilz.com"#
            }
            0x9E65 => {
                r#"LTE-WLAN Aggregation Adaptation Protocol (LWAAP), used by LTE-WLAN Aggregation (LWA) as defined in 3GPP TS 36.300 http://www.3gpp.org/DynaReport/36300.htm"#
            }
            0xA0ED => {
                r#"When carried over layer 2 technologies such as Ethernet, this ethertype
will be used to identify IPv6 datagrams using LoWPAN encapsulation as defined
in IETF RFC 4944 "Transmission of IPv6 Packets over IEEE 802.15.4 Networks""#
            }
            0xA85A => {
                r#"This Ethertype will be used to identify a new type of protocol used for device to device communication in Wireless LAN"#
            }
            0xA8C8 => {
                r#"The Virtual Link Control (VLC) protocol for Ethernet-based subscriber access networks. The VLC protocol is specified in IEEE Std. 1904.2. For more information, visit https://www.ieee1904.org/2"#
            }
            0xAAC5 => r#"For implementation of protocol still in development"#,
            0xAB37 => {
                r#"Please read draft-ietf-bier-mpls-encapsulation-12 at https://datatracker.ietf.org/doc/draft-ietf-bier-mpls-encapsulation/.

"#
            }
            0xAD3F => {
                r#"The protocol allows PON OLTs to coordinate scheduling and other PON management functions across multiple OLTs on the same layer-2 domain."#
            }
            0xAEFE => {
                r#"The protocol for this Ethertype is specified by CPRI Industry Cooperation and found at http://www.cpri.info"#
            }
            0xB081 => {
                r#"WiTSnet is a real-time Ethernet communication protocol with flexible networking, multiple reliability guarantees, low latency and high synchronisation accuracy, application layer compatibility and extensibility, and simple deployment and easy maintenance."#
            }
            0xB298 => r#"Kinova Discovery protocol"#,
            0xB382 => r#"Private Protocol"#,
            0xB4E3 => r#"Use for varies special communications on various projects"#,
            0xB62C => {
                r#"OPC UA over Time Sensitive Networking (TSN) adds deterministic communication capabilities to the OPC UA standard."#
            }
            0xB6FE => {
                r#"Routing functionality through branch controllers is added to the EtherCAT technology by the new EtherCAT G enhancement. IP routing is not available and VLAN based routing does not meet the requirements, so we need an own Ethertype for that purpose."#
            }
            0xB7EA => {
                r#"The Ethertype will be used to identify a “Channel” in which control messages are encapsulated as payload of GRE packets. When a GRE packet tagged with the EtherType is received, the payload will be handed to the network processor for processing."#
            }
            0xBC17 => {
                r#"This Ethertype is used to identify a protocol used for automatic configuration of STBs on the LANs"#
            }
            0xC111 => {
                r#"A simple protocol will be used to manage and monitor a device or group of devices."#
            }
            0xC9D1 => {
                r#"Legacy assignment (use 8870 instead) - LLC encapsulation as defined by IEEE Std 802.1AC-2016."#
            }
            0xCCB1 => {
                r#"A customer Tag. This tag carries information indicating the enabling and related operations for congestion control schemes."#
            }
            0xCCE0 => {
                r#"Reliable Internet Stream Transport is an open source, open specification transport protocol designed for reliable transmission of video over lossy networks (including the internet) with low latency and high quality."#
            }
            0xCEB4 => {
                r#"Various control, management, and transport protocols delineated by sub-types that are administered and managed by General Motors."#
            }
            0xD28B => {
                r#"We plan to use this Ethertype for multiple proprietary protocols.  They are still under development. We will disclose the protocols publicly once they are generally available."#
            }
            0xD672 => {
                r#"sFlow is is a multi-vendor measurement technology for sampling packets in Ethernet devices."#
            }
            0xDC94 => {
                r#"Requirements:
- One-to-one and one-to-many communication, with per-class configurable priority, reliability, order-preservation and rate limiters
- End-to-end hop-to-hop and IS-IS Cyber Security
- Seamless redundancy
- Time management
- Datagram-based
"#
            }
            0xDDB3 => {
                r#"This Ethertype enables high-speed deterministic transport of OT communications through corporate IT Ethernet. For VLAN tagged frames, the Ethertype appears after the VLAN tag header, and is followed by an SEL-managed sub-type field."#
            }
            0xE23B => {
                r#"Media Access Control (MAC) Privacy protection protocol as specified in IEEE Std 802.1AE."#
            }
            0xE890 => r#"Telecor Inc. eSeries Communication Protocol"#,
            0xEADD => {
                r#"This Ethertype will be used to identify a new type of network protocol supporting the multi-labels communication pattern and its comprehensive interoperation."#
            }
            0xEC19 => r#"proprietary"#,
            0xED3E => {
                r#"EtherType is FoRCES inter-FE LFB type.
Contains packet-processing meta-data followed by encapsulated packet. See https://datatracker.ietf.org/doc/draft-ietf-forces-interfelfb/ "#
            }
            0xF1C1 => r#"Redundancy tag (R-TAG) as defined in IEEE Std 802.1CB"#,
            0xF45F => {
                r#"This Ethertype enables IDP (Identifier Protocol), which was defined in ITU-T Y.3075. IDP is responsible for maintaining the rules and regulations about how to process the address/es in the packet and modifying the address field in the packet."#
            }
            0xF4C4 => {
                r#"The protocol interconnects devices for acquiring, recording, analyzing and processing measurement data."#
            }
            0xF5D2 => {
                r#"This Ethertype enables cryptographically protected transport of OT communications. For VLAN tagged frames, the Ethertype appears after the VLAN tag header, and is followed by an SEL-managed sub-type field."#
            }
            0xF68E => {
                r#"This is a fieldbus protocol, especially for the motion control systems which require precise synchronization between the nodes."#
            }
            0xF8DB => {
                r#"Real time protocol for control and safety functions for elevators and escalators"#
            }
            0xFC0F => r#"Proprietary protocol."#,
            0xFC3D => r#"Radio over Ethernet (RoE) encapsulation per IEEE Std 1914.3."#,
            _ => return None,
        })
    }

    /// The organization that registered the [`EtherType`] sourced from the [IEEE Registration Authority](https://standards.ieee.org/develop/regauth).
    pub const fn ieee_organization(self) -> Option<&'static str> {
        Some(match self.0 {
            0x0101..=0x01FF => r#"Xerox (Experimental)"#,
            0x0200..=0x0201
            | 0x0400
            | 0x0600
            | 0x0804..=0x0805
            | 0x0807
            | 0x0844
            | 0x0888..=0x088A
            | 0x0A00..=0x0A01
            | 0x1000..=0x10FF
            | 0x2000..=0x207F
            | 0x6010..=0x6014
            | 0x8006
            | 0x8008
            | 0x8013..=0x8016
            | 0x801C
            | 0x8035..=0x8036
            | 0x8044
            | 0x8046..=0x8047
            | 0x8049
            | 0x805B..=0x805D
            | 0x8060
            | 0x8062
            | 0x8065..=0x806A
            | 0x806C..=0x8077
            | 0x807A..=0x8083
            | 0x809B..=0x809F
            | 0x80A3..=0x80B3
            | 0x80C0..=0x80D4
            | 0x80DE..=0x80E3
            | 0x80F2..=0x80F5
            | 0x80F7
            | 0x8130..=0x8136
            | 0x8139..=0x813D
            | 0x8148..=0x814B
            | 0x8150..=0x8153
            | 0x815C..=0x815E
            | 0x8164..=0x8166
            | 0x817D..=0x818D
            | 0x819A..=0x81A4
            | 0x81CC..=0x81DD
            | 0x81E6..=0x81F8
            | 0x8263..=0x826A
            | 0x827F..=0x8282
            | 0x829A..=0x82AB
            | 0x8390
            | 0x8694..=0x86A1
            | 0x86A3..=0x86AC
            | 0x86DB
            | 0x86DE
            | 0x86E0..=0x86EF
            | 0x8700..=0x8720
            | 0x8725..=0x8728
            | 0x8739..=0x873C
            | 0x8755..=0x875C
            | 0x8780..=0x8785
            | 0x884C
            | 0x8856
            | 0x887E
            | 0x8A96..=0x8A97
            | 0x9000
            | 0xFF00..=0xFF0F => r#"Private"#,
            0x0500 => r#"University  of Berkeley"#,
            0x0660..=0x0661 => r#"Dlog"#,
            0x0800..=0x0803 | 0x0A00..=0x0A01 | 0x8283 | 0x8F00..=0x8FFF => r#"Xerox"#,
            0x0806 => r#"Symbolics, Inc."#,
            0x0808 => r#"Xerox (Frame Relay ARP) RFC1701"#,
            0x081C => r#"Symbolics Private"#,
            0x0842
            | 0x22E2
            | 0x22E7
            | 0x22E9..=0x22EA
            | 0x8100
            | 0x888E
            | 0x88A8
            | 0x88B5..=0x88B7
            | 0x88CC
            | 0x88E7
            | 0x88F5..=0x88F6
            | 0x8902
            | 0x8929
            | 0x893F..=0x8940
            | 0x894B
            | 0x8952
            | 0x89A2
            | 0xC9D1
            | 0xE23B
            | 0xF1C1 => r#"IEEE 802.1 Chair"#,
            0x0884 => r#"AES DAta"#,
            0x0885 => r#"Hastech"#,
            0x0886..=0x0887 => r#"Tolerant Systems"#,
            0x088B..=0x089A => r#"Sentry/Schlumberger"#,
            0x0900 => r#"Ungermann-Bass*"#,
            0x0B00..=0x0B07 => r#"Universitiy of Berkeley"#,
            0x0B01..=0x0B07 | 0xC020..=0xC02F | 0xC220..=0xC22F => r#"University of Berkeley"#,
            0x0BAD..=0x0BAF => r#"Banyan VINES"#,
            0x1600 => r#"Valid Systems"#,
            0x22DF | 0x888B => r#"Toshiba Corporation"#,
            0x22E0 => r#"NETCORE TNDUSTRAIL CO.LTD"#,
            0x22E1 => r#"Sandvine Incorporated"#,
            0x22E3 => r#"Marvell Semiconductor, Inc."#,
            0x22E4 => r#"Aprius"#,
            0x22E5 => r#"Gigamon"#,
            0x22E6 => r#"Nethra Imaging Incorporated"#,
            0x22E8 => r#"RAD Data Communications, Ltd."#,
            0x22EB
            | 0x883D..=0x8843
            | 0x88BB
            | 0x88BE
            | 0x8903..=0x8906
            | 0x8909
            | 0x8921
            | 0x8926
            | 0x894F => r#"Cisco Systems, Inc"#,
            0x22EC => r#"KERI(Korea Electrotechnology Research Institute)"#,
            0x22ED..=0x22EE => r#"Huawei Technologies (Netherlands) B.V."#,
            0x22EF => r#"Anagran, Inc"#,
            0x22F0 => r#"	IEEE 1722 Working Group"#,
            0x22F1..=0x22F2 => r#"Association of Radio Industries and Businesses (ARIB)"#,
            0x22F3..=0x22F4 | 0x893B | 0x8946 => r#"IETF TRILL Working Group"#,
            0x3181 => r#"VG Data Systems"#,
            0x3C10..=0x3C20 | 0x9001..=0x9003 => r#"3Com"#,
            0x4242 => r#"PCS Basic Block Protocol"#,
            0x4400..=0x4409 => r#"Computer Generation, Inc."#,
            0x454C => r#"SR RESEARCH LTD"#,
            0x5000..=0x5009 | 0x8159..=0x815B => r#"Intel"#,
            0x5208 => r#"BBN Simnet"#,
            0x6000..=0x6009 | 0x8038..=0x8042 => r#"DEC"#,
            0x6558 => r#"Trans Ether Bridging"#,
            0x6559 => r#"Raw Frame Relay"#,
            0x7000..=0x7004 => r#"Ungermann-Bass"#,
            0x7010..=0x7019 => r#"Olteco (Olevetti)"#,
            0x7020..=0x7029 => r#"LRT"#,
            0x7030..=0x7034 => r#"InterLan"#,
            0x7035..=0x7036 => r#"Three Rivers"#,
            0x7677 | 0x8104..=0x810F => r#"Locus Computing Corp."#,
            0x8001 => r#"Computervision"#,
            0x8002 => r#"Cromemco"#,
            0x8003..=0x8004 => r#"BBN"#,
            0x8005 | 0x81DE..=0x81E0 | 0x8206 | 0x8859 => r#"Hewlett Packard"#,
            0x8009 => r#"Textronix"#,
            0x800A | 0x80F6 | 0x873D..=0x8746 => r#"Siemens  AG"#,
            0x800B..=0x800E => r#"Technology Concepts Inc."#,
            0x800F => r#"Exxon"#,
            0x8010 => r#"Excelan"#,
            0x8011 => r#"Accutest"#,
            0x8012 => r#"National Semiconductor"#,
            0x8017..=0x8018 | 0x801F => r#"Able Computers"#,
            0x8019 => r#"Apollo Computers"#,
            0x801A => r#"Spartacus Computers"#,
            0x801B => r#"Lawrence Labs"#,
            0x801D => r#"Megatek Corporation"#,
            0x801E | 0x881B..=0x881F => r#"Sun Microsystems"#,
            0x8020 => r#"Mosaic Technologies"#,
            0x8021 => r#"Matsuhita Graphic"#,
            0x8022 => r#"Data General"#,
            0x8023..=0x802D => r#"Metaphor"#,
            0x802E => r#"Tymshare"#,
            0x802F => r#"Tigan, Inc."#,
            0x8030 => r#"Plexus Computers"#,
            0x8031 => r#"PCS Systemtechnik GmbH"#,
            0x8032 => r#"Codex Corporation"#,
            0x8033 => r#"VIA Systems"#,
            0x8034 => r#"PIXEL COMPUTER INC."#,
            0x8037 => r#"GenRad"#,
            0x8043 => r#"Valid Logic Systems"#,
            0x8045 => r#"HYDRA COMPUTER SYSTEMS INC."#,
            0x8048 => r#"Cygnet Systems"#,
            0x804A => r#"Middle East Tech. Univ."#,
            0x804B..=0x805A => r#"Stanford Telecomm., Inc."#,
            0x805E..=0x805F => r#"Perq Data Systems"#,
            0x8061 => r#"VTA Technologies, Inc."#,
            0x8063..=0x8064 => r#"Arbat (UK) Limited"#,
            0x806B => r#"Ridge Computers"#,
            0x8078 => r#"Accel Technologies"#,
            0x8079 | 0x8121..=0x812A => r#"Philips International B.V."#,
            0x8086 => r#"Intel Americas, Inc."#,
            0x80A0 => r#"Convergent Technologies"#,
            0x80A1..=0x80A2 => r#"Xerox Corporation"#,
            0x80B4 => r#"Hellige GMBH"#,
            0x80B5..=0x80BF => r#"American Info. Tech. Inc."#,
            0x80D5..=0x80DC => r#"IBM Corp."#,
            0x80DD => r#"Varian Associates"#,
            0x80F1 => r#"Xyvision, Inc."#,
            0x80FA..=0x80FE => r#"Metier Management"#,
            0x80FF..=0x8103 => r#"WellFleet Communications"#,
            0x8101..=0x8103 => r#"Nortel Networks"#,
            0x8105..=0x8106 => r#"Gandalf"#,
            0x810A..=0x810E => r#"Autotote Limited"#,
            0x8110..=0x8111 => r#"Alsys"#,
            0x8112..=0x8113 => r#"Micrognosis International"#,
            0x8114..=0x8118 => r#"Software Consulting Services"#,
            0x8119..=0x8120 => r#"Omnex Corporation"#,
            0x812B..=0x812F | 0x8223..=0x8227 => r#"Talaris Systems Inc."#,
            0x8137..=0x8138 => r#"Novell, Inc."#,
            0x813E..=0x8142 => r#"Micronetics Design Corp."#,
            0x8143..=0x8144 => r#"Diamond Sales Ltd."#,
            0x8145..=0x8147 => r#"Vrije Universiteit"#,
            0x814C => r#"Rensselaer Technology Park"#,
            0x814D..=0x814E => r#"BIIN"#,
            0x814F => r#"Technically Elite Concepts, Inc."#,
            0x8154..=0x8156 => r#"BTS, Inc."#,
            0x8157 => r#"Castelle"#,
            0x8158 => r#"DSIR"#,
            0x815F..=0x8163 => r#"Mass. General Hospital"#,
            0x8167..=0x816C => r#"Keithley Instruments, Inc."#,
            0x816D..=0x8171 => r#"DSC Communications Corp."#,
            0x8172 => r#"Hirschmann Automation and Control GmbH"#,
            0x8173..=0x817C => r#"The NTI Group"#,
            0x8191 => r#"Performance Technology"#,
            0x8192..=0x8199 => r#"Dow Chemical U.S. A."#,
            0x81A5..=0x81AE => r#"RAD Network Devices Inc."#,
            0x81AF..=0x81B6 => r#"Chipcom Corporation"#,
            0x81B7..=0x81B9 | 0x8208..=0x821B => r#"Xyplex, Inc."#,
            0x81BA..=0x81C1 => r#"Saguaro Software, Ltd."#,
            0x81C2..=0x81CB => r#"Atlantix Corporatiion"#,
            0x81E1..=0x81E5 => r#"System Designers Software Inc."#,
            0x81F9..=0x8202 | 0x882A => r#"Cabletron Systems, Inc."#,
            0x8203..=0x8205 => r#"Quantum Software Systems, Ltd."#,
            0x8207 => r#"SynOptics"#,
            0x8221..=0x8222 => r#"Ascom Banking Systems Ltd."#,
            0x8228..=0x822B => r#"Virtual Machine Research Inc."#,
            0x822C..=0x8235 => r#"Tandem Computers"#,
            0x8236..=0x823D => r#"Thomson Sintra"#,
            0x823E..=0x8240 => r#"Advanced Encryption Systems, Inc."#,
            0x8241..=0x8242 => r#"Compex"#,
            0x8243..=0x8260 => r#"The Royal Hong Kong Jockey Club"#,
            0x8261..=0x8262 => r#"Channel Systems International, Inc."#,
            0x826B..=0x826C => r#"Legent Corporation"#,
            0x826D..=0x8274 => r#"Extra Document"#,
            0x8275..=0x827E => r#"Fischer & Porter Co."#,
            0x8284..=0x8285 => r#"Kalpana"#,
            0x8286..=0x8289 => r#"Wang Laboratories, Inc."#,
            0x828A..=0x8299 => r#"Network-1, Inc."#,
            0x82AC..=0x838F | 0x8391..=0x8693 => r#"Walker Richer & Quinn, Inc."#,
            0x86A2 => r#"Persoft, Inc."#,
            0x86AD..=0x86B0 => r#"Star Gate Technologies"#,
            0x86B1..=0x86B6 => r#"Goodyear Technical Center"#,
            0x86B7..=0x86BA => r#"DigiBoard"#,
            0x86BB..=0x86DA => r#"Software Technology, Inc."#,
            0x86DC => r#"Oak Solutions Ltd."#,
            0x86DD => r#"USC/ISI"#,
            0x86DF => r#"University of SC - ISI"#,
            0x86F0..=0x86FF => r#"Starlight Networks, Inc."#,
            0x8721..=0x8724 => r#"Marconi Simulation"#,
            0x8729..=0x8730 => r#"Telxon"#,
            0x8731..=0x8738 => r#"Combinet, Inc."#,
            0x8747..=0x8750 => r#"Hybrid"#,
            0x8751..=0x8754 => r#"Digital Ocean"#,
            0x875D..=0x8766 => r#"The University of Utah"#,
            0x8767..=0x876A => r#"Metricom, Inc."#,
            0x876B..=0x876F | 0x8847..=0x8848 | 0x8939 => r#"Cisco Systems"#,
            0x8770..=0x877F | 0x8861..=0x8862 | 0x886D..=0x886E => r#"Intel Corporation"#,
            0x8786..=0x878F => r#"CMG"#,
            0x8790 => r#"LaserMaster Corporation"#,
            0x8791..=0x8795 => r#"3COM"#,
            0x8796..=0x87FA => r#"Qualcomm Inc."#,
            0x87FB..=0x87FD => r#"RUN-Rad Unlimited Networking"#,
            0x87FE..=0x8802 => r#"Network Intelligence Inc."#,
            0x8803 => r#"Quantel Limited"#,
            0x8804..=0x8807 | 0x884B => r#"Stratus Technologies"#,
            0x8808..=0x880A => r#"IEEE 802.3 Working Group"#,
            0x880B => r#"US Robotics Corporation"#,
            0x880C | 0x8824 => r#"Ipsilon Networks, Inc."#,
            0x880D => r#"Furuno Electric Co, Ltd."#,
            0x880E => r#"Light & Sound Design, Ltd."#,
            0x880F..=0x8812 => r#"Hypercom Network Systems"#,
            0x8813 => r#"silex technology, Inc."#,
            0x8816 => r#"Znyx Corporation"#,
            0x8817..=0x8818 => r#"Advanced Micro Devices"#,
            0x8819 => r#"Peak Audio"#,
            0x881A | 0x884A => r#"NBase Communications"#,
            0x8820 => r#"APRESIA Systems Ltd"#,
            0x8821 => r#"Apple, Inc."#,
            0x8822 => r#"Wind River Systems"#,
            0x8823 => r#"WaveSpan Corporation"#,
            0x8825..=0x8828 => r#"No Wires Needed BV"#,
            0x8829 => r#"e-Net Inc."#,
            0x882B..=0x882D => r#"Assured Access Technology, Inc."#,
            0x882E => r#"Berkeley Networks"#,
            0x882F => r#"Intecom Inc."#,
            0x8830..=0x8831 => r#"Host Automation Products"#,
            0x8833..=0x883C => r#"Excel Inc."#,
            0x8844 => r#"Alcatel Telecom"#,
            0x8845 => r#"3Com Primary Access"#,
            0x8846 => r#"Multimeg Electronique"#,
            0x8849 => r#"ATM Forum Technical Committee"#,
            0x884D => r#"Klos Technologies, Inc."#,
            0x884E => r#"ESP/ANTEC"#,
            0x884F..=0x8852 => r#"Juniper Networks"#,
            0x8853..=0x8854 => r#"Japan Digital Laboratory Co., Ltd."#,
            0x8855 => r#"Ramat Gabriel Industrial Park"#,
            0x8857 => r#"Xinex"#,
            0x8858 => r#"Adtech"#,
            0x885A => r#"Foundry Networks"#,
            0x885B..=0x885C => r#"Philips Medizin Systeme Boblingen GmbH"#,
            0x885D => r#"Endocardial Solutions, Inc."#,
            0x885E => r#"Quantum Corporation"#,
            0x885F | 0x88D3 => r#"Digidesign"#,
            0x8860 => r#"AGCS"#,
            0x8863..=0x8864 => r#"UUNET Technologies, Inc."#,
            0x8865 => r#"Visual Networks"#,
            0x8866..=0x8867 | 0x8877 => r#"Lucent Technologies"#,
            0x8868 => r#"NBX Corporation"#,
            0x8869 => r#"BigBand Networks"#,
            0x886A => r#"Electronic Theatre Controls, Inc."#,
            0x886B => r#"Bay Networks"#,
            0x886C => r#"Epigram, Inc."#,
            0x886F | 0x88C0 => r#"Microsoft Corporation"#,
            0x8870 | 0x88E5 | 0x8910 => r#"IEEE 802.1 Working Group"#,
            0x8871 => r#"Nippon Telegraph and Telephone Corporation"#,
            0x8872 => r#"UUNET Technologies"#,
            0x8873 => r#"Crescent Networks Inc."#,
            0x8874 | 0x888A => r#"Broadcom"#,
            0x8875 => r#"Adaptive Broadband Corporation"#,
            0x8876 => r#"OpenDOF Project, Inc."#,
            0x8878 => r#"Lucent Technologies Inc."#,
            0x8879 => r#"Expand Networks"#,
            0x887A => r#"Marconi Communications Inc."#,
            0x887B => r#"Intellon Corporation"#,
            0x887C | 0x8883 => r#"2Wire Inc"#,
            0x887D => r#"Nishan Systems, Inc."#,
            0x887F => r#"ESI"#,
            0x8880 => r#"Exbit Technology"#,
            0x8881 => r#"TIA"#,
            0x8882 => r#"Comtrol Europe Ltd"#,
            0x8884 => r#"The ATM Forum"#,
            0x8885 => r#"NETSEC"#,
            0x8886 => r#"Big Band Networks Ltd"#,
            0x8887 => r#"Alloptic Inc."#,
            0x8888 => r#"Santera Systems"#,
            0x8889 => r#"PolyTrax Information Technology AG"#,
            0x888C => r#"Inetcam Inc."#,
            0x888D => r#"T11 Technical Committee"#,
            0x888F => r#"Lincom Wireless"#,
            0x8890 => r#"Force 10 Networks"#,
            0x8891 => r#"Thunder River Technologies, Inc."#,
            0x8892 => r#"PROFIBUS International"#,
            0x8893 => r#"Mobile Internet Services, Inc."#,
            0x8894 => r#"Texas Instruments"#,
            0x8895 | 0x8925 => r#"General Dynamics Mission Systems"#,
            0x8896 => r#"Digigram"#,
            0x8897 => r#"Phonex Broadband"#,
            0x8898 => r#"Aspen Networks Inc."#,
            0x8899 => r#"Realtek Semiconductor Corp."#,
            0x889A => r#"Data Storage Institute"#,
            0x889B => r#"Mindspeed Technologies"#,
            0x889C => r#"Hatteras Networks"#,
            0x889D => r#"Occam Networks"#,
            0x889E => r#"Remotec, Inc."#,
            0x889F => r#"Transition Networks"#,
            0x88A0 => r#"WideBand Corporation"#,
            0x88A1 => r#"Telkonet Inc."#,
            0x88A2 | 0x893C | 0x893E => r#"Coraid Inc."#,
            0x88A3 => r#"Airspan Communications Limited"#,
            0x88A4 => r#"Beckhoff Automation GmbH & Co KG"#,
            0x88A5 => r#"Jedai Broadband Networks"#,
            0x88A6 => r#"Cetacean Networks, Inc."#,
            0x88A7 => r#"HUAWEI TECHNOLOGIES CO.,LTD"#,
            0x88A9 => r#"Meshnetworks, Inc."#,
            0x88AA | 0x8901 => r#"Nokia Networks/IP Mobility Networks"#,
            0x88AB => r#"B&R Industrial Automation GmbH "#,
            0x88AC => r#"Tenor Networks, Inc."#,
            0x88AD => r#"XiMeta Technology Americas Inc."#,
            0x88AE => r#"Andiamo Systems, Inc."#,
            0x88AF => r#"Life Racing Limited"#,
            0x88B0..=0x88B1 => r#"Cranite Systems, Inc."#,
            0x88B2 => r#"DragonWave Inc."#,
            0x88B3 => r#"Cirronet, Inc."#,
            0x88B4 => r#"Instant Wireless Network Communications, Co. Ltd."#,
            0x88B8..=0x88BA => r#"IEC TC57"#,
            0x88BC => r#"International Telecommunication Union - Telecommunication St"#,
            0x88BD => r#"Atheros Communications, Inc."#,
            0x88BF => r#"Proxim Corporation"#,
            0x88C1 => r#"Avaya Inc"#,
            0x88C2 => r#"LEA (Laboratoire Europeen ADSL)"#,
            0x88C3 => r#"Infineon Technologies Corporate Research ST"#,
            0x88C4 => r#"AutoCell Laboratories, Inc."#,
            0x88C5 => r#"KoolSpan, Inc."#,
            0x88C6 => r#"3eti"#,
            0x88C7 | 0x890D => r#"IEEE 802.11 Working Group"#,
            0x88C8 => r#"Sony Interactive Entertainment Inc."#,
            0x88C9 => r#"Xeline Co., Ltd."#,
            0x88CA => r#"Ericsson Research Canada Inc"#,
            0x88CB | 0x8907 => r#"Zhejiang University"#,
            0x88CD => r#"sercos international e.V."#,
            0x88CE => r#"Level 5 Networks, Inc."#,
            0x88CF => r#"Panasonic Mobile Communications Co.,Ltd."#,
            0x88D0 => r#"Nortel Wireless"#,
            0x88D1 => r#"Integral Access Inc."#,
            0x88D2 => r#"3GPP2"#,
            0x88D4 => r#"Contemporary Control Systems, Inc."#,
            0x88D5 => r#"GGSN Design & Development"#,
            0x88D6 | 0x88EA..=0x88EB => r#"Hewlett Packard Enterprise"#,
            0x88D7 => r#"Vienna University of Technology"#,
            0x88D8 | 0x88EE => r#"Metro Ethernet Forum"#,
            0x88D9 => r#"Microsoft"#,
            0x88DA => r#"Entropic Communication Inc"#,
            0x88DB | 0x9999 => r#"Adtran Inc"#,
            0x88DC => r#"IEEE P1609 WG"#,
            0x88DD => r#"Audio Engineering Society, Inc."#,
            0x88DE => r#"Akimbi Systems, Inc."#,
            0x88E0 | 0x88E6 => r#"Nortel"#,
            0x88E1 => r#"HomePlug Powerline Alliance, Inc."#,
            0x88E2 => r#"JAPAN RADIO CO., LTD."#,
            0x88E3 => r#"SIEMENS AG"#,
            0x88E4 => r#"VIA Technologies, Inc."#,
            0x88E8 => r#"LVL7 Systems"#,
            0x88E9 => r#"Motorola"#,
            0x88EC => r#"Talari Networks, Inc."#,
            0x88ED => r#"Meshcom Technologies, Inc"#,
            0x88EF => r#"NVIDIA Corporation"#,
            0x88F0 => r#"IEEE P1451.0"#,
            0x88F1 => r#"Rajant Corporation"#,
            0x88F2 => r#"Honeywell"#,
            0x88F3 => r#"AlaxalA Networks Corporation"#,
            0x88F4 => r#"Woodhead Software & electronics"#,
            0x88F7 => r#"IEEE I&M Society TC9"#,
            0x88F8 => r#"DMTF"#,
            0x88F9 => r#"Alcatel-Lucent Italia"#,
            0x88FA => r#"VARAN-BUS-NUTZERORGANISATION"#,
            0x88FB => r#" International Electrotechnical Commission"#,
            0x88FC => r#"Accedian Network"#,
            0x88FD => r#"Beceem Communications, Inc"#,
            0x88FE => r#"LS Industrial Systems. Co. Ltd."#,
            0x88FF => r#"FUJITSU LIMITED"#,
            0x8900 => r#"Red Lion"#,
            0x8908 => r#"Waves Audio LTD"#,
            0x890A => r#"WIT"#,
            0x890B => r#"Panduit Corp"#,
            0x890C => r#"FibroLAN Ltd."#,
            0x890E => r#"ads-tec GmbH"#,
            0x890F => r#"MITSUBISHI ELECTRIC CORPORATION NAGOYA WORKS"#,
            0x8911 => r#"Enea"#,
            0x8912 => r#"Gigle Semiconductor"#,
            0x8913 => r#"Rohde & Schwarz SIT GmbH"#,
            0x8914 | 0x8928 => r#"Brocade Communications Systems LLC"#,
            0x8915 | 0x8932 | 0x8948..=0x8949 | 0x8CE4 => r#"Mellanox Technologies, Inc."#,
            0x8916 => r#"Element Labs"#,
            0x8917 => r#"IEEE 802.21 Working Group"#,
            0x8918 => r#"Hangzhou H3C Technologies Co., Limited"#,
            0x8919 => r#"Xsigo Systems"#,
            0x891A => r#"Mobile Wireless Group"#,
            0x891B => r#"InfoSpark Technology Institute"#,
            0x891C => r#"Martin Professional A/S"#,
            0x891D => r#"TTTech Computertechnik AG"#,
            0x891E => r#"CipherOptics"#,
            0x891F => r#"British Telecommunications Plc."#,
            0x8920 => r#"XStreamHD"#,
            0x8922 => r#"VMware, Inc."#,
            0x8923 | 0x8934 => r#"Phoenix Contact GmbH & Co. KG"#,
            0x8924 => r#"iris-GmbH infrared & intelligent sensors"#,
            0x8927 => r#"Klavis Technologies"#,
            0x892A => r#"Roland SG Corporation"#,
            0x892B | 0xDDB3 | 0xF5D2 => r#"Schweitzer Engineering Laboratories, Inc."#,
            0x892C => r#"China Broadband Wireless IP Standard group(ChinaBWIPS)"#,
            0x892D => r#"Bachmann electronic GmbH"#,
            0x892E => r#"Ozmo Devices"#,
            0x892F => r#"International Electrotechnical Commission"#,
            0x8930 => r#"Google, Inc."#,
            0x8931 => r#"IC Plus Corp."#,
            0x8933 => r#"Infinera, Inc."#,
            0x8935 => r#"V.D.S. Video Display Systems srl"#,
            0x8936 => r#"Ericsson AB"#,
            0x8937 => r#"Panasas"#,
            0x8938 => r#"HDBaseT Alliance"#,
            0x893A => r#"IEEE 1905.1"#,
            0x893D => r#"Korea Electric Power Corporation"#,
            0x8941 => r#"Cambium Networks"#,
            0x8942 => r#"Big Switch Networks"#,
            0x8943 => r#"Tektronix  Communications"#,
            0x8944 => r#"Carallon Ltd"#,
            0x8945 => r#"Crypto AG"#,
            0x8947 | 0x9E65 => r#"ETSI"#,
            0x894A => r#"Endace Technologies Ltd."#,
            0x894C => r#"IEC – International Electrotechnical Commission"#,
            0x894D => r#"Softing Industrial Automation GmbH"#,
            0x894E => r#"FSM Solutions Limited"#,
            0x8950 => r#"ISO TC204 WG16"#,
            0x8951 => r#"Nokia Solutions and Networks GmbH & Co. KG"#,
            0x8953 => r#"Embedtronics Oy"#,
            0x8954 => r#"Datasat Technologies"#,
            0x8955 => r#"Megapixel VR"#,
            0x8956 => r#"Japan Cable Television Engineering Association"#,
            0x8BC2 => r#"KATIM L.L.C"#,
            0x9040..=0x905F => r#"TopWare/Grand Computer Corp."#,
            0x909B => r#"VisionVera Information Technology Company, Ltd"#,
            0x9433 => r#"O-RAN Alliance e.V."#,
            0x96B6 | 0xA85A | 0xCCB1 => r#"Huawei Technologies Co., Ltd."#,
            0x99FE => r#"ASAM e.V."#,
            0x9A22 => r#"IETF"#,
            0x9AC6 => r#"Tesla,Inc."#,
            0x9C40 => r#"Pilz GmbH & Co. KG"#,
            0xA0ED => r#"IETF 6lo working group"#,
            0xA580 => r#"Siemens Gammasonics"#,
            0xA8C8 => r#"IEEE 1904 Access Networks Working Group"#,
            0xAAC5 => r#"Analog Devices Inc"#,
            0xAB37 | 0xED3E => r#"IETF Routing Area"#,
            0xAD3F => r#"Tibit Communications"#,
            0xAEFE => r#"CPRI Cooperation c/o Ericsson AB"#,
            0xB081 => r#"Shenzhen Inovance Technology Co.,Ltd"#,
            0xB298 => r#"Kinova Robotics"#,
            0xB382 => r#"Cisco Systems, Inc."#,
            0xB4E3 => r#"Cable Television Laboratories, Inc (CableLabs)"#,
            0xB62C => r#"OPC Foundation"#,
            0xB6FE => r#"EtherCAT Technology Group"#,
            0xB7EA | 0xEADD => r#"Huawei Technologies, Co., Ltd"#,
            0xBC17 => r#"Dish Technologies Corp"#,
            0xBC19 => r#"DarkMatter L.L.C"#,
            0xC111 => r#"Soraa, Inc."#,
            0xCCE0 => r#"Video Services Forum, Inc."#,
            0xCEB4 => r#"General Motors"#,
            0xD28B => r#"Arista Networks"#,
            0xD672 => r#"InMon Corp."#,
            0xDC94 => r#"Kone Corporation"#,
            0xE890 => r#"Telecor Inc."#,
            0xEC19 => r#"AirTies Wireless Networks"#,
            0xF45F => r#"Beijing Haiwang Network Technologies Co., Ltd"#,
            0xF4C4 => r#"iba AG"#,
            0xF68E => r#"YASKAWA ELECTRIC CORPORATION"#,
            0xF8DB => r#"Schindler Elevator Ltd."#,
            0xFC0F => r#"Senetas Corporation Ltd"#,
            0xFC3D => r#"IEEE 1914 Next Generation Fronthaul Interface  Working Group"#,
            0xFEA0..=0xFEAF => r#"NTT Electronics Technology Corp."#,
            0xFFFF => r#"RFC1701"#,
            _ => return None,
        })
    }

    /// The address of the organization that registered the [`EtherType`] sourced from the [IEEE Registration Authority](https://standards.ieee.org/develop/regauth).
    pub const fn ieee_organization_address(self) -> Option<&'static str> {
        Some(match self.0 {
            0x0101..=0x01FF | 0x0A00 => r#" Webster NY US  "#,
            0x0200..=0x0201
            | 0x0400
            | 0x0600
            | 0x0804..=0x0805
            | 0x0807
            | 0x0844
            | 0x0888..=0x088A
            | 0x0A00..=0x0A01
            | 0x1000..=0x10FF
            | 0x2000..=0x207F
            | 0x6010..=0x6014
            | 0x8006
            | 0x8008
            | 0x8013..=0x8016
            | 0x801C
            | 0x8035..=0x8036
            | 0x8044
            | 0x8046..=0x8047
            | 0x8049
            | 0x805B..=0x805D
            | 0x8060
            | 0x8062
            | 0x8065..=0x806A
            | 0x806C..=0x8077
            | 0x807A..=0x8083
            | 0x809B..=0x809F
            | 0x80A3..=0x80B3
            | 0x80C0..=0x80D4
            | 0x80DE..=0x80E3
            | 0x80F2..=0x80F5
            | 0x80F7
            | 0x8130..=0x8136
            | 0x8139..=0x813D
            | 0x8148..=0x814B
            | 0x8150..=0x8153
            | 0x815C..=0x815E
            | 0x8164..=0x8166
            | 0x817D..=0x818D
            | 0x819A..=0x81A4
            | 0x81CC..=0x81DD
            | 0x81E6..=0x81F8
            | 0x8263..=0x826A
            | 0x827F..=0x8282
            | 0x829A..=0x82AB
            | 0x8390
            | 0x8694..=0x86A1
            | 0x86A3..=0x86AC
            | 0x86DB
            | 0x86DE
            | 0x86E0..=0x86EF
            | 0x8700..=0x8720
            | 0x8725..=0x8728
            | 0x8739..=0x873C
            | 0x8755..=0x875C
            | 0x8780..=0x8785
            | 0x884C
            | 0x8856
            | 0x887E
            | 0x8A96..=0x8A97
            | 0x9000
            | 0xFF00..=0xFF0F => r#""#,
            0x0500 => r#"Computer Science Department Berkeley CA US 94720 "#,
            0x0660..=0x0661 => r#"Gmbh D-8037 Olching  DE  "#,
            0x0800..=0x0803
            | 0x081C
            | 0x0900
            | 0x0A01
            | 0x0B00..=0x0B07
            | 0x0BAD..=0x0BAF
            | 0x1600
            | 0x4242
            | 0x5000..=0x5009
            | 0x5208
            | 0x6558..=0x6559
            | 0x7000..=0x7004
            | 0x7010..=0x7019
            | 0x800F
            | 0x8017..=0x8018
            | 0x801F
            | 0x8021
            | 0x8038..=0x8042
            | 0x805E..=0x805F
            | 0x80A1..=0x80A2
            | 0x8241..=0x8242
            | 0x826D..=0x8274
            | 0x8283
            | 0x880D
            | 0x8816
            | 0x8849
            | 0x8F00..=0x8FFF
            | 0xA580
            | 0xC020..=0xC02F
            | 0xC220..=0xC22F
            | 0xFFFF => r#"   US  "#,
            0x0806 => r#"243 Vassar Street Cambridge  US 02139 "#,
            0x0808 => r#", Webster NY US 14580 "#,
            0x0842
            | 0x22E2
            | 0x22E7
            | 0x22E9..=0x22EA
            | 0x8100
            | 0x8870
            | 0x888E
            | 0x88A8
            | 0x88B5..=0x88B7
            | 0x88CC
            | 0x88F5..=0x88F6
            | 0x8902
            | 0x8929
            | 0x893F..=0x8940
            | 0x894B
            | 0x8952
            | 0x89A2
            | 0xC9D1
            | 0xE23B
            | 0xF1C1 => r#" c/o RAC Administrator , IEEE Piscataway NJ US 08554 "#,
            0x0884 => r#"1900 Minnesota Court Mississauga Ontario CA L5N 3C9 "#,
            0x0885 => r#"Hastech, Inc. Manchester NH US 03101 "#,
            0x0886..=0x0887 => r#"81 East Daggett Drive San Jose CA US 95134 "#,
            0x088B..=0x089A => r#"1601 Technology Drive San  US  "#,
            0x22DF => {
                r#"Transmission Distribution & Industrial System Company Tokyo  JP 105-8001 "#
            }
            0x22E0 => {
                r#"the 8th floor,block C5,stage 2, Tianfu software park Gaoxin Area,Cheng Du city SiChuan CN 610041 "#
            }
            0x22E1 => r#"408 Albert St. Waterloo Ontario CA N2L 3V3 "#,
            0x22E3 => r#"5488 Marvell Lane Santa Clara CA US 95054 "#,
            0x22E4 => r#"440 N Wolfe Rd Sunnyvale CA US 94085 "#,
            0x22E5 => r#"598 Gibraltar Drive Milpitas California US 95035 "#,
            0x22E6 => r#"2855 Bowers Ave Santa Clara CA US 95051 "#,
            0x22E8 => r#"24 Raoul Wallenberg St. Tel Aviv  IL 69719 "#,
            0x22EB => r#"170 West Tasman Drive San Jose CA US 95134 "#,
            0x22EC => r#"1271-19 Ansan-city Gyeonggi-do KR 426-170 "#,
            0x22ED..=0x22EE => r#"Karspeldreef 4 Amsterdam Z-O  NL 1101 CJ "#,
            0x22EF => r#"580 North Pastoria Ave. Sunnyvale CA US 94085 "#,
            0x22F0 => r#"1722 Chair c/o IEEE Piscataway NJ US 08854 "#,
            0x22F1..=0x22F2 => r#"Nittochi Bldg. 11F, Tokyo  JP 100-0013 "#,
            0x22F3..=0x22F4 | 0x893B | 0x8946 | 0xA0ED => {
                r#"c/o Internet Society Reston VA US 20190-5108 "#
            }
            0x3181 => r#"St. George's Court Chesire  GB WA14 5UG "#,
            0x3C10..=0x3C1F => r#"5400 Bayfront Plaza Santa Clara CA US 95052 "#,
            0x3C20 | 0x9001..=0x9003 => r#"3 Com Corporation Boxborough MA US 01719 "#,
            0x4400..=0x4409 => r#"3855 Presidential Pkwy Atlanta GA US 30340 "#,
            0x454C => r#"SR Research Ltd. Mississauga Ontario CA L4X 1R3 "#,
            0x6000..=0x6009 => r#"1925 Andover St. Tewksbury MA US 01876 "#,
            0x7020..=0x7029 => r#" Reading Berks, England GB  "#,
            0x7030..=0x7034 => r#"160 Turnpike Road Chelmsford MA US 01824 "#,
            0x7035..=0x7036 => r#"Three Rivers Computer Corporation Pittsburgh PA US 15230 "#,
            0x7677 | 0x8104..=0x810F => r#"3330 Ocean Park Boulevard Santa Monica CA US 90405 "#,
            0x8001 => r#"14 Crosby Drive Bedford MA US 01730 "#,
            0x8002 => r#"280 Bernardo Avenue Mountain View CA US 94043 "#,
            0x8003..=0x8004 => r#"50 Moulton Street Cambridge MA US 02238 "#,
            0x8005 => r#"Compaq drive Palo Alto CA US 77070 "#,
            0x8009 => r#"PO Box 500 Beaverton Oregon US 97077 "#,
            0x800A | 0x80F6 | 0x873D..=0x8746 => r#"Otto-Han-Ring-6 Munich  DE D-81739 "#,
            0x800B..=0x800E => r#"730 Boston Post Road Sudbury Massachusetts US 01776 "#,
            0x8010 => r#"1599 Flicker Ave. San Jose CA US 95131 "#,
            0x8011 => r#"25 Industrial Avenue Chelmsford MA US 01824 "#,
            0x8012 => r#"2900 Semiconductor Drive Santa Clara CA US 95051 "#,
            0x8019 => r#"330 Billerica Road Chelmsford MA US 01824 "#,
            0x801A => r#"847 Rogers St. Lowell MA US 01852-4337 "#,
            0x801B => r#"Lawrence Livermore National Laboratory Livermore CA US 94550 "#,
            0x801D => r#"16868 Via Del Campo Court San Diego CA US 92127 "#,
            0x801E => r#"2550 Gracia Ave. Mountain View CA US 94043 "#,
            0x8020 => r#"47 Manning Road Billerica MA US 01821-3970 "#,
            0x8022 => r#"4400 Computer Drive Westboro MA US 01580 "#,
            0x8023..=0x802D => r#"2500 Garcia Avenue Mountain View CA US 94043 "#,
            0x802E => r#"Network Technology Division  Cupertino US CA "#,
            0x802F => r#" Palo Alto CA US  "#,
            0x8030 => r#"2230 Martin Avenue Santa Clara CA US 95050 "#,
            0x8031 => r#"600 Suffold St Lowell MA US 01854 "#,
            0x8032 => r#"20 Cabot Blvd. Mansfield MA US 02048 "#,
            0x8033 => r#"76  Treble Cove Road N. Billerica MA US 08162 "#,
            0x8034 => r#"260 Fordham Road Wilmington MA US 01887 "#,
            0x8037 => r#"Component Test Division Bolton MA US 01740 "#,
            0x8043 => r#"1395 Charleston Road Mountain View CA US 94043 "#,
            0x8045 => r#"12 Mercer Road Natwick MA US 01760 "#,
            0x8048 => r#"610 Palomar Avenue Sunnyvale CA US 94086 "#,
            0x804A => r#"Department of Computer Engineering Ankara  TR  "#,
            0x804B..=0x805A => r#"2421 Mission College Blvd. Santa Clara CA US 95050 "#,
            0x8061 => r#"2040 Sherman Street Hollywood FL US 33020 "#,
            0x8063..=0x8064 => r#"Kent House Ashford Kent TN23 1PJ GB  "#,
            0x806B => r#"2451 Mission College Blvd. Santa Clara CA US 95054 "#,
            0x8078 => r#"7358 Trade St. San Diego CA US 92121 "#,
            0x8079 => r#"Oude ApelDoornseweg 41-45   NL  "#,
            0x8086 => r#"1906 Fox Drive Champaign IL US 61820 "#,
            0x80A0 => r#"2700 North 1st St San Jose CA US 95150-6685 "#,
            0x80B4 => r#"Heinrich-von-Stephan-Str. 4  West Gernany DE  "#,
            0x80B5..=0x80BF => r#"10201 Torre Avenue Cupertino CA US 95014 "#,
            0x80D5..=0x80DC => r#"PO Box 12195 Research Triangle Park NC US 27709 "#,
            0x80DD => r#"611 Hansen Way Palo Alto CA US 94303 "#,
            0x80F1 => r#"101 Edgewater Drive Wakefield MA US 01880 "#,
            0x80FA..=0x80FE => r#"3 Foundation Street Ipswich Suffolk GB 1P4 1BG Engl "#,
            0x80FF..=0x8103 => r#"12 DeAngelo Drive Bedford MA US 01730 - 2204 "#,
            0x8101..=0x8103 => r#"8200 Dixie Rd Brampton Ontario CA 0000 "#,
            0x8105..=0x8106 => r#"9 Slack Road Nepean Ontario CA K2G 0B7 "#,
            0x810A..=0x810E => r#"One Hundred Bellevue Road Newark Delaware US 19714 - 6009 "#,
            0x8110..=0x8111 => r#"29, Avenue do Versailles 78170 La Celle Saint-Cloud  FR  "#,
            0x8112..=0x8113 => r#"160 Queen Victoria St. London EC4V 4DA GB  "#,
            0x8114..=0x8118 => r#"3162 Bike Path Nazareth PA US 18064 "#,
            0x8119..=0x8120 => r#"2483 Old Middlefield Way Mountain View CA US 94043 "#,
            0x8121..=0x812A => r#"Building SFF-3.32, P.O Box 80002 NL-5600 JB EINDHOVEN  NL  "#,
            0x812B..=0x812F => r#"PO Box 261580 San Diego CA US 92126 "#,
            0x8137..=0x8138 => r#"122 EAST 1700 SOUTH Provo UT US  84606 "#,
            0x813E..=0x8142 => r#"5 Choke Cherry Rd Rockville MD US 20850 "#,
            0x8143..=0x8144 => r#"17, Charterhouse Street ECIN 6RA England GB  "#,
            0x8145..=0x8147 => r#"De Boelelaan 1081 Amsterdam  NL  "#,
            0x814C => r#"165 Jordan Road Troy New York US 12180 "#,
            0x814D..=0x814E => r#" Hillsboro OR US  "#,
            0x814F => r#" Torrance CA US  "#,
            0x8154..=0x8156 => r#"2300 South 2300 St. Salt Lake City UT 84119 US  "#,
            0x8157 => r#"2540 Mission College Blvd. Santa Clara CA US 95054 "#,
            0x8158 => r#"Dept. of Scientific & Industrial Researc PO Box 31-311 Lower Hutt NZ  "#,
            0x8159..=0x815B => r#"2402 W. Beardsley Road Phoenix AZ US 85027 "#,
            0x815F..=0x8163 => r#"Cardiac Computer Ctr. Boston MA US 02114 "#,
            0x8167..=0x816C => r#"Systems Division Cleveland OH US 44139 "#,
            0x816D..=0x8171 => r#"3040 North First Street San Jose CA US 95134 "#,
            0x8172 => r#"Stuttgarter Straße 45-51 Neckartenzlingen  DE D-72654 "#,
            0x8173..=0x817C => r#"801 Lincoln Road Santa Clara CA US 95051 "#,
            0x8191 => r#"801 Lincoln Center San Antonio TX US 78230 "#,
            0x8192..=0x8199 => r#"1101 Building, Michigan Division Midland MI US 48667 "#,
            0x81A5..=0x81AE => r#"4455 Torrance Blvd. Torrance CA US 90503 "#,
            0x81AF..=0x81B6 => r#"195 Bear Hill Road Waltham MA US 02154 "#,
            0x81B7..=0x81B9 => r#"0000 Concord MA US 0000 "#,
            0x81BA..=0x81C1 => r#"Box 36508 Tucson AZ US 85740 "#,
            0x81C2..=0x81CB => r#"5401 NW Broken Sound Blvd. Boca Raton FLA US 33431 "#,
            0x81DE..=0x81E0 => r#"1501 Page Mill Road Palo Alto CA US 94304 "#,
            0x81E1..=0x81E5 => r#"9 New England Executive Park Burlington MA US 01803 "#,
            0x81F9..=0x8202 => r#"35 Industrial Way Rochester NH US 03867 "#,
            0x8203..=0x8205 => r#"175 Terence Matthews Cresent Ottawa Ontario CA K2M 1W8 "#,
            0x8206 => r#"Filton Rd Bristol  BS12 6QZ England GB  "#,
            0x8207 => r#"4401 Great America Parkway Santa Clara CA US 95054 "#,
            0x8208..=0x821B => r#"330 Codman Hill Road Boxborough MA US 01719 "#,
            0x8221..=0x8222 => r#" Solothurn  CH  "#,
            0x8223..=0x8227 => r#"6059 Cornerstone Court W. San Diego CA US 92196 "#,
            0x8228..=0x822B => {
                r#"#28-1141 Eagleridge Dr. Port Coquitlam British Columbia, Canaca CA V3E 1K1 "#
            }
            0x822C..=0x8235 => r#"14231 Tendem Blvd. Austin TX US 78728 - 6610 "#,
            0x8236..=0x823D => r#"525, route des Dolines BP 138-06561 Valbinne Cedex FR  "#,
            0x823E..=0x8240 => r#" Santa Clara CA US  "#,
            0x8243..=0x8260 => r#"Information Sys. Dept. Happy Valley  HK  "#,
            0x8261..=0x8262 => r#"93 So. La Patera Lane Santa Barbara CA US 93117 "#,
            0x826B..=0x826C => r#"LANSpy Development Manager Pittsburg PA US  15212 "#,
            0x8275..=0x827E => r#"East County Line Rd Westminster PA US 18974 "#,
            0x8284..=0x8285 => r#"125 Nicholson Ln San Jose CA US 95124 "#,
            0x8286..=0x8289 => r#"M/S 013-390 Lowell MA US 95124 "#,
            0x828A..=0x8299 => r#"PO Box 8370 Long Island City NY US 11101 "#,
            0x82AC..=0x838F | 0x8391..=0x8693 => r#" Seattle WA US  "#,
            0x86A2 => r#"465 Science Dr Madison WI US 53744-4953 "#,
            0x86AD..=0x86B0 => r#"29300 Aurora Road Solon OH US 44139 "#,
            0x86B1..=0x86B6 => r#"D/451A Akron OH US 44309-3531 "#,
            0x86B7..=0x86BA => r#"6400 Flying Coul Dr. Eden Prarie MN US 55344 "#,
            0x86BB..=0x86DA => r#"1511 Park Ave Melbourne FL US 32901 "#,
            0x86DC => r#"Robin Enterprise Centre, Suite 25 West Yorkshire England GB ED 10 9 TE "#,
            0x86DD => r#"4676 Admiralty Way Marina del Rey CA US 90292 "#,
            0x86DF => r#"4676 Admiralty Way Marina del Rey CA US 90292-6696 "#,
            0x86F0..=0x86FF => r#"444 Castro Street Mountain View CA US 94041 "#,
            0x8721..=0x8724 => {
                r#"Fulmar Way, Donibristle Industrial Park Fife Ky11 5JX Scotland GB  "#
            }
            0x8729..=0x8730 => r#"3330 West Market St Akron OH US 44334 "#,
            0x8731..=0x8738 => r#"333 West El Camino Real Sunnyvale CA US 94087 "#,
            0x8747..=0x8750 => r#"20863 Stevens Creek Blvd. Cupertino CA US 95014-8750 "#,
            0x8751..=0x8754 => r#"35 Corporate Woods Overland Park KS US 66210 "#,
            0x875D..=0x8766 => r#"Dept of Computer Science Salt Lake City UT US 84112 "#,
            0x8767..=0x876A => r#"980 University Avenue Los Gatos CA US 95030 "#,
            0x876B..=0x876F => r#"1525 O'Brien Dr Menlo Park CA US 94025 "#,
            0x8770..=0x877F => r#"2111 NE 25 Ave Hillsboro OR US 97124 "#,
            0x8786..=0x878F => r#"Laan van Kronenburg 14 Postbus 412   NL  "#,
            0x8790 => r#"7156 Shady Oak Rd Eden Prairie MN US 55344 "#,
            0x8791..=0x8795 => r#"5400 Bayfront Plaza Santa Clara CA US 95052-8145 "#,
            0x8796..=0x87FA => r#"6455 Lusk Blvd. San Diego CA US 92121-2779 "#,
            0x87FB..=0x87FD => r#"31 Habarzel St. Tel Aviv 69710  IL  "#,
            0x87FE..=0x8802 => r#"1804 Embarcadero Road Palo Alto CA US 94303 "#,
            0x8803 => r#"Turnpike Road Newbury, Berkshire England GB RG13 2NE "#,
            0x8804..=0x8807 | 0x884B => r#"5 Mill and Main Place, Suite 500 Maynard MA US 01754 "#,
            0x8808..=0x880A => {
                r#"IEEE 802.3 Chair, c/o RAC Administrator, IEEE  Piscataway NJ US 08854 "#
            }
            0x880B => r#"1300 E. Woodfield Roar, Suite: 506 Schaumburg IL US 60173 "#,
            0x880C | 0x8824 => r#"2191 E. Bayshore Rd Palo Alto CA US 94303 "#,
            0x880E => r#"201 Coventry Rd EH30 9TG  GB  "#,
            0x880F..=0x8812 => r#" Phoenix AZ US  "#,
            0x8813 => r#"2-3-1 Hikaridai, Seika-cho, Souraku-gun Kyoto  JP 619-0237 "#,
            0x8817..=0x8818 => r#"1 AMD Place Sunnyvale CA US 94088 "#,
            0x8819 => r#"1790 30 St Boulder CO US 80301 "#,
            0x881A | 0x884A => r#"8943 Fullbright Ave Chatsworth CA US 91311 "#,
            0x881B..=0x881F => r#"2550 Garcia Ave Mountain View CA US 415-786-6636 "#,
            0x8820 => {
                r#"Tsukuba Network Technical Center, Kidamari 3550 Tsuchiura-shi Ibaraki-ken JP 300-0026 "#
            }
            0x8821 => r#"1 Infinite Loop Cupertino CA US 95014 "#,
            0x8822 => r#"1010 Atlantic Ave. Alameda CA US 94501 "#,
            0x8823 => r#"967 N. Shoreline Blvd. Mountain View CA US 94043 "#,
            0x8825..=0x8828 => r#"Spuistraat 73   NL  "#,
            0x8829 => r#"12325 Hymeadow Dr. Austin TX US 78750 "#,
            0x882A => r#"PO Box 5005 Rochester NH US 03866-5005 "#,
            0x882B..=0x882D => r#"48499 Milmont Dr Fremont CA US 94538 "#,
            0x882E => r#"683 River Oaks Parkway San Jose CA US 95134 "#,
            0x882F => r#"System Software Group Dallas TX US 75248 "#,
            0x8830..=0x8831 => r#"200 East Main Johnson City TN US 37604 "#,
            0x8833..=0x883C => r#"255 Independence Dr. Hyannis MA US 02601 "#,
            0x883D..=0x8843 | 0x8926 => r#"170 West Tasman Drive San Jose CA US 95134-1706 "#,
            0x8844 => r#"Francis Wellesplein 1   BE  "#,
            0x8845 => r#"12230  World Trade Drive San Diego CA US 92128 "#,
            0x8846 => r#"3675 Grande-Allee J7H 1H5  CA  "#,
            0x8847..=0x8848 => r#"1414 Massachusetts Ave. Boxborough MA US 01719 "#,
            0x884D => r#"604 Daniel Webster Highway Merrimack NH US 03054 "#,
            0x884E => r#"4920 Avalon Ridge Parkway   US  "#,
            0x884F..=0x8852 => r#"1133 Innovation Way Sunnyvale CA US 94089 "#,
            0x8853..=0x8854 => {
                r#"Hardware Development Headquarter Kawasaki-shi Kanagawa-ken 125 JP  "#
            }
            0x8855 => r#"Lan Optics Building   IL 10551 "#,
            0x8857 => r#"2-1520 Cliveden Av New Westminster B. C.  CA V3M 6J8 "#,
            0x8858 => r#"3465 Waialae Ave Honolulu Hawaii US 96816 "#,
            0x8859 => r#"8000 Foothills Blvd. Roseville CA US 95747 "#,
            0x885A => r#"680 W. Maude Ave. Sunnyvale CA US 94086 "#,
            0x885B..=0x885C => r#"Hewlett-Packard-Straße 2 Böblingen  DE 71034 "#,
            0x885D => r#"1450 Energy Lane St. Paul MN US 55108 "#,
            0x885E => r#"500 McCarthy Blvd. Milpitas CA US 95035 "#,
            0x885F => r#"3401-A Hillview Ave. Palo Alto CA US 94034 "#,
            0x8860 => r#"2500 W. Utopia Rd. Phoenix AZ US 85072-2179 "#,
            0x8861..=0x8862 => r#"2111 NE 25th Ave. Hillsborough OR US 97124 "#,
            0x8863..=0x8864 => r#"3060 Williams Drive Fairafx CA US 22031-4648 "#,
            0x8865 => r#"2092 Gaither Road Rockville MD US 20850 "#,
            0x8866..=0x8867 => r#"1100 E. Warrenville Rd. Naperville IL US 60566 "#,
            0x8868 => r#"100 Brickstone Square Andover MA US 01810 "#,
            0x8869 => r#"475 Broadway Street Redwood City CA US 94065 "#,
            0x886A => r#"3031 Pleasant View Rd Middleton WI US 53562 "#,
            0x886B => r#"PO Box 58185 Santa Clara CA US 95052-8185 "#,
            0x886C => r#"870 West Maude Ave. Sunnyvale CA US 94086 "#,
            0x886D..=0x886E => r#"JF3-410 Hillsboro OR US 97124 "#,
            0x886F | 0x88C0 => r#"One Microsoft Way Redmond WA US 98052 "#,
            0x8871 => r#"Software Laboratory Tokyo  JP 180-8585 "#,
            0x8872 => r#"100 Manhattanville Rd. Purchase NY US 10577 "#,
            0x8873 => r#"201 Riverneck Road Chelmsford MA US 01842 "#,
            0x8874 => r#"2099 Gateway Place San Jose CA US 95110 "#,
            0x8875 => r#"175 Science Parkway Rochester NY US 14620 "#,
            0x8876 => r#"3855 SW 153rd Drive Beaverton OR US 97003 "#,
            0x8877..=0x8878 => r#"260 14th Street NW Atlanta GA US 30318 "#,
            0x8879 => r#"Atidim Tech Park - Bldg. 4 TelAviv  IL 61580 "#,
            0x887A => r#"8616 Freeport Parkway Irving TX US 75063 "#,
            0x887B => r#"5100 West Silver Springs Blvd. Ocala FL US 34482 "#,
            0x887C | 0x8883 => r#"694 Tasman Ave. Milpitas CA US 95035 "#,
            0x887D => r#"3850 North First Street San Jose CA US 95134-1702 "#,
            0x887F => r#"3701 E. Plano Parkway Plano TX US 75074 "#,
            0x8880 => r#"Hoerkaer 18  Herlen DK 2730 "#,
            0x8881 => r#"2500 Wilson Boulevard Arlington VA US 22201 "#,
            0x8882 => r#"The Courtyard Studio Laughton Oxfordshire GB OX6 0EE "#,
            0x8884 => r#"2570 West El Camino Real Mountainview CA US 94040-1313 "#,
            0x8885 => r#"13505 Dulles Technology Dr., Ste. 1 Herdon VA US 20171 "#,
            0x8886 => r#"3 Azrieli Towers Tel Aviv  IL 67093 "#,
            0x8887 => r#"6960 Koll Center Parkway Pleasanton CA US 94566 "#,
            0x8888 => r#"2901 Summit Ave. Plano TX US 75074 "#,
            0x8889 => r#"Martin-Kollar-Str. 5 Munchen  US D-81819 "#,
            0x888A => r#"4385 River Green Parkway Deluth GA US 30096 "#,
            0x888B => {
                r#"Micro Electronics & Systems Companents Department Fuchu-shi Tokyo JP 183-8511 "#
            }
            0x888C => r#"10171 Pacific Mesa Blvd. San Diego CA US 92121 "#,
            0x888D => r#"Sponsored by:  NCITS Secretariat Washington DC US 2005 "#,
            0x888F => r#"5120 W. Goldleaf Circle Los Angeles CA US 90056 "#,
            0x8890 => r#"350 Holger Way San Jose CA US 95134 "#,
            0x8891 => r#"23 Corporate Plaza Ste. 250 Newport Beach CA US 92660 "#,
            0x8892 => r#"Haid-und-Neu-Straße 7 Karlsruhe  DE D-76131 "#,
            0x8893 => r#"KS Building 2F, 1-17-8, Bunkyo-ku Tokyo JP 113-0025 "#,
            0x8894 => r#"12500 TI Blvd Dallas TX US 75243 "#,
            0x8895 | 0x8925 => r#"150 Rustcraft Road Dedham MA US 02026 "#,
            0x8896 => r#"Parc de Pre Milliet   FR  "#,
            0x8897 => r#"6952 So. High Tech Dr. Midvale UT US 84047 "#,
            0x8898 => r#"4655 Old Ironsides Drive Santa Clara CA US 95054 "#,
            0x8899 => r#"No. 2 Industry E. Rd. IX, Hsinchu  TW 30077 "#,
            0x889A => r#"DSI Building, 5 Engineering Drive 1 Kent Ridge Crescent NUS SG 117608 "#,
            0x889B => r#"Les Taissounieres HB1 Sophia Antipolis Cedex  US 06903 "#,
            0x889C => r#"P.O. Box 10025 Research Triangle Park NC US 27709-0025 "#,
            0x889D => r#"77 Robin Hill Rd. Santa Barbara CA US 93117 "#,
            0x889E => r#"114 Union Valley Road Oakridge TN US 37830 "#,
            0x889F => r#"6475 City West Parkway Eden Prarie MN US 55344 "#,
            0x88A0 => r#"401 W. Grand Gallatin MO US 64640 "#,
            0x88A1 => r#"20374 Seneca Meadows Pkwy Germantown MD US 21401 "#,
            0x88A2 => r#"565 Research Dr. Athens GA US 30605 "#,
            0x88A3 => r#"Cambridge House Uxbridge  GB UB8 1UN "#,
            0x88A4 => r#"Huelshorstweg 20 Verl  DE D-33415 "#,
            0x88A5 => r#"331 Newman Springs Rd. Red Bank NJ US 07701 "#,
            0x88A6 => r#"100 Arboretum Drive Portsmouth NH US 03801 "#,
            0x88A7 => r#"8F, Huadian R&D Building Shenzhen Guangdong CN 518129 "#,
            0x88A9 => r#"485 Keller Road Maitland FL US 32751 "#,
            0x88AA => r#"P.O. Box 301   FI 00045 "#,
            0x88AB => r#" B&R Strasse 1  Eggelsberg        AT 5142 "#,
            0x88AC => r#"100 Nagog Park Acton MA US 01720-3409 "#,
            0x88AD => r#"54 Ray Street New Brunswick NJ US 08901 "#,
            0x88AE => r#"375 E. Tasman Dr. San Jose CA US 95134 "#,
            0x88AF => r#"Unit 6 Repton Close Basildon Essex US SS13 1LE "#,
            0x88B0..=0x88B1 => r#"6620 Via Del Oro San Jose CA US 95119 "#,
            0x88B2 => r#"600-411 Lesset Dr. Kanata Ontario CA K2L 2R8 "#,
            0x88B3 => r#"3079 Premiere Pkwy Duluth GA US 30097 "#,
            0x88B4 => r#"4F X Xietong Building, No. 12 Gaoxin 2nd Xi' an,  US 710075 "#,
            0x88B8..=0x88BA => r#"3, rue de Varembre Geneva 20  CH CH-1211 "#,
            0x88BB | 0x88BE | 0x8909 => r#"170 W. Tasman Drive San Jose CA US 95134 "#,
            0x88BC => r#"Place des Nations Geneva 20  CH CH-1211 "#,
            0x88BD => r#"529 Almanor Avenue Sunnyvale CA US 94085-3512 "#,
            0x88BF => r#"28 Bay View Road Wellesley MA US 02482 "#,
            0x88C1 => r#"360 Mt Kemble Ave Morristown NJ US 07960 "#,
            0x88C2 => r#"9 rue des Charmilles Cesson SEV1 GNE Cedex FR 35577 "#,
            0x88C3 => r#"M/S Geb.:  10-574 Munchen  DE D-61739 "#,
            0x88C4 => r#"125 Nagog Park Dr. Acton MA US 01720 "#,
            0x88C5 => r#"11134 Stephalee Lane North Bethesda MD US 20852 "#,
            0x88C6 => r#"700 Kings Farm Blvd Rockville MD US 20850 "#,
            0x88C7 | 0x890D => r#"c/o RAC Administrator Piscataway  NJ US 08854 "#,
            0x88C8 => r#"1-7-1 Konan Minato-ku Tokyo JP 108-0075 "#,
            0x88C9 => r#"7F. Chungjin Bldg., 475-22, Seoul  KR 137-819 "#,
            0x88CA => r#"8400 Decarie Blvd Montreal Québec CA H4P2N2 "#,
            0x88CB => r#"Institute of Advanced Process Control Hangzhou Zhejiang CN 310027 "#,
            0x88CD => r#"Küblerstrasse 1 Süssen Baden-Württemberg DE 73079 "#,
            0x88CE => r#"840 W. California Ave., STE 240 Sunnyvale CA US 94086 "#,
            0x88CF => r#"600, Saedo-cho, Tsuzuki-ku Yokohama Kanagawa JP 224-8539 "#,
            0x88D0 => r#"Mesh Networks Nepean/Ottawa Ontario CA K2H-8E9 "#,
            0x88D1 => r#"6 Omni Way Chelmsford MA US 01824 "#,
            0x88D2 => r#"1960 Lucent Lane Naperville IL US 60566 "#,
            0x88D3 => r#"2001 Junipero Serra Blvd Daly City CA US 94014 "#,
            0x88D4 => r#"2431 Curtiss Street Downers Grove Illinois US 60515 "#,
            0x88D5 => r#"2201 Lakeside Blvd. Richardson Texas US 75082 "#,
            0x88D6 | 0x88EA..=0x88EB => r#"3333 Scott Blvd 6280 America Center Dr CA US 95002 "#,
            0x88D7 => r#"Treitlstraße 3/3 Vienna  AT 1040 "#,
            0x88D8 | 0x88EE => r#"1300 Bristol St. N. Suite 160 Newport Beach CA US 92660 "#,
            0x88D9 => r#"1 Microsoft Way Redmond WA US 98052 "#,
            0x88DA => r#"9276 Scranton Rd. Suite 200 San Diego CA US 92121 "#,
            0x88DB | 0x9999 => r#"901 Explorer Blvd. Huntsville AL US 35806 "#,
            0x88DC => r#"3800 N Fairfax Drive #207 Arlington VA US 22203-1759 "#,
            0x88DD => r#"60 East 42nd Street, Ste. 2520 New York NY. US 10165-2520 "#,
            0x88DE => r#" San Mateo CA US 94114-2060 "#,
            0x88E0 => r#"3500 Carling Avenue Nepean Ontario US K1Y 4H7 "#,
            0x88E1 => r#"2400 Camino Ramon, #375 San Ramon  US 94583 "#,
            0x88E2 => r#"5-1-1 Shimorenjaku Mitaka-City Tokyo JP 181-8510 "#,
            0x88E3 => r#"Oestliche Rheinbrückenstraße 50 Karlsruhe Baden-Württemberg DE 76181 "#,
            0x88E4 => r#"8F, 533, Chung-Cheng Road Hsin-Tien Taipei TW 231 "#,
            0x88E5 | 0x8910 => {
                r#"IEEE 802.1 Chair, c/o RAC Administrator IEEE Piscataway NJ US 08854  "#
            }
            0x88E6..=0x88E7 => r#"M/S P7903B12 Santa Clara CA US 95054 "#,
            0x88E8 => r#"100 Perimeter Park Drive, Suite H Morrisville NC US 27560 "#,
            0x88E9 => r#"120 Turnpike Road Southborough MA US 02062 "#,
            0x88EC => r#"150 W.Iowa Ave #208 Sunnyvale CA US 94086 "#,
            0x88ED => r#"Meritullinkatu 1 C Helsinki  FI 00170 "#,
            0x88EF => r#"2701 San Tomas Expressway Santa Clara CA US 95050 "#,
            0x88F0 => r#"700 King Farm Blvd. Rockville MD US 20850 "#,
            0x88F1 => r#"400 East King Street Malvern PA US 19355 "#,
            0x88F2 => r#"512 Virginia Dr Ste A Fort Washington PA US 19034 "#,
            0x88F3 => {
                r#"Shinkawasaki Mitsui Building West Tower 13F Kawasaki Kanagawa JP 212-0058 "#
            }
            0x88F4 => r#"50 Northland Road Waterloo Ontario CA N2V 1N3 "#,
            0x88F7 => r#"100 Bureau Drive Gaithersburg MD US 20899-8220 "#,
            0x88F8 => r#"225 SE Main St. Portland OR US 97214 "#,
            0x88F9 => r#"Via Trento, 30 Vimercate   MONZA-BRIANZA 20871  IT  "#,
            0x88FA => r#"Buermoserstraße 10 LAMPRECHTSHAUSEN SALZBURG AT A-5112 "#,
            0x88FB => r#"3, rue de Varembé  GENEVA 20  CH CH - 1211 "#,
            0x88FC => r#"2351 boul. Alfred-Nobel St-Laurent QC CA H4S 2A9 "#,
            0x88FD => r#"3960 Freedom Circle, Santa Clara Ca US 95054 "#,
            0x88FE => r#"533, Hogye-dong Dongan-gu Anayang-si Gyeonggi-do KR 431-080 "#,
            0x88FF => r#"403, Kosugi-cho 1-chome, Nakahara-ku Kawasaki Kanagawa JP 211-0063 "#,
            0x8900 => r#"820 S. University Blvd. Mobile AL US 36609 "#,
            0x8901 => r#"P.O.BOX 370 Espoo  FI FI-00045 Nok "#,
            0x8903..=0x8906 => r#"170 W Tasman Drive San Jose CA US 95134 "#,
            0x8907 => {
                r#"Institute of Advanced Process Control,Zhejiang University Hangzhou Zhejiang CN 310027 "#
            }
            0x8908 => r#"Azrilei Center 3, Triangle tower Tel Aviv Central IL 67023 "#,
            0x890A => r#"138 Avenue Leon Beranger St-Laurent-Du-Var France FR 06706 "#,
            0x890B => r#"17301 Ridgeland Ave Tinley Park IL US 60477 "#,
            0x890C => r#"2 Hacarmel st., POB 544 Yokneam Illit  IL 20692 "#,
            0x890E => r#"Raiffeisenstraße 14 Leinfelden BW DE 70771 "#,
            0x890F => r#"1-14, Yada-minami 5-chome, Higashi-ku Nagoya Aichi JP 461-8670 "#,
            0x8911 => r#"1100 Park St NE Washington DC US 20002 "#,
            0x8912 => r#"Barcelona Activa- Vivero Glorias Barcelona  ES 08018 "#,
            0x8913 => r#"Am Studio 3 Berlin  DE 12489 "#,
            0x8914 | 0x8928 => r#"1320 Ridder Park Dr San Jose CA US 95131 "#,
            0x8915 | 0x8932 | 0x8949 => {
                r#"350 Oakmead Parkway, Suite 100   Sunnyvale CA US 94085 "#
            }
            0x8916 => r#"3350 Scott Blvd Santa Clara CA US 95054 "#,
            0x8917 => r#"IEEE 802.21 Chair, c/o RAC Administrator IEEE Piscataway NJ US 08854 "#,
            0x8918 => {
                r#"Oriental Electronics Bldg., #2, Chuangye RoadÂ£Â¬Shangdi Information Industry Base,Haidian District, Beijing Beijing CN 100085 "#
            }
            0x8919 => r#"70 West Plumeria Drive San Jose CA US 95134 "#,
            0x891A => r#"2111 NE 25th Ave Hillsboro OR US 97124 "#,
            0x891B => {
                r#"Room 1907, Quantum Plaza, 27 Zhichun Road, Haidian District, Beijing  CN 100083 "#
            }
            0x891C => r#"Olof Palmes Allé 18 Århus N Midtjylland DK 8200 "#,
            0x891D => r#"Schoenbrunner Strasse 7 Vienna  AT 1040 "#,
            0x891E => r#"701 Corporate Center Drive Raleigh NC US 27607 "#,
            0x891F => r#"81 Newgate Street  London GB EC1A 7AJ "#,
            0x8920 => r#"7900B Westpark Drive McLean VA US 22102 "#,
            0x8921 | 0xB382 => r#"3850 Zanker Rd. San Jose CA US 95134 "#,
            0x8922 => r#"3401 Hillview Ave Palo Alto CA US 94304 "#,
            0x8923 | 0x8934 => r#"Dringenauer Strasse 30 Bad Pyrmont Lower Saxony DE D-31812 "#,
            0x8924 => r#"Ostendstraße 1-14 Berlin  DE 12459 "#,
            0x8927 => r#"9A Bld de France Braine-L'Alleud Brabant BE 1420 "#,
            0x892A => r#"4010-5 Wada Matsumoto-shi Nagano JP 390-1242 "#,
            0x892B | 0xDDB3 | 0xF5D2 => r#"2350 NE Hopkins Court Pullman WA US 99163 "#,
            0x892C => r#"100191-006#  Beijing  CN 100191 "#,
            0x892D => r#"Kreuzackerweg 33 Feldkirch  AT 6800 "#,
            0x892E => r#"2595 E. Bayshore Rd. Suite 100 Palo Alto CA US 94303 "#,
            0x892F => r#"3, rue de Varembé Geneva GE CH CH-1211 "#,
            0x8930 => r#"1600 Amphitheatre Parkway  Mountain View CA US 94043 "#,
            0x8931 => r#"10F, No. 47, Lane 2, Kwang-Fu Road,  Hsin-Chu city  TW 30071 "#,
            0x8933 => r#"169 Java Drive Sunnyvale CA US 94089 "#,
            0x8935 => r#"Via del Pantano 71 SCANDICCI FI IT 50018 "#,
            0x8936 => r#"Flöjelbergsgatan 2a Mölndal  SE SE-431 84 "#,
            0x8937 => r#"969 W. Maude Ave. Sunnyvale CA US 94085 "#,
            0x8938 => r#"3855 SW 153rd Drive Beaverton OR US 97006 "#,
            0x8939 => r#"1414 Massachusetts Ave.  Boxborough MA US 01719 "#,
            0x893A => r#"445 Hoes Lane Piscataway  NJ US 08855-1331 "#,
            0x893C | 0x893E => {
                r#"255 Shoreline Drive, Suite 650 Redwood City CA - California US 94065 "#
            }
            0x893D => r#"105 Munji-Ro Yuseong-Gu Daejeon KR 305-760 "#,
            0x8941 => r#"Linhay Business Park Ashburton Devon GB TQ13 7UP "#,
            0x8942 => r#"100 West Evelyn Ave. Mountain View CA US 94041 "#,
            0x8943 => r#"3033 W. President George Bush Hwy Plano TX US 75075  "#,
            0x8944 => r#"Studio G20, Shepherds Building  London GB W140DA "#,
            0x8945 => r#"Zugerstrasse 24 Steinhausen Zug CH 6312 "#,
            0x8947 | 0x9E65 => r#"650 Route des lucioles Sophia antipolis  FR 06921 "#,
            0x8948 | 0x8CE4 => r#"350 Oakmead Parkway, Suite 100  Sunnyvale CA US 94085 "#,
            0x894A => r#"85 Alexandra Street Hamilton  NZ 3204 "#,
            0x894C => r#"3, rue de Varembé Geneva GE CH 1211 "#,
            0x894D => r#"Richard Reitzner Allee 6 Haar Bavaria DE 85540 "#,
            0x894E => r#"Granatny per 2/9-73 Moscow Moscow RU 123001 "#,
            0x894F => r#"3800 Zankar Rd San Jose CA US 95134 "#,
            0x8950 => r#"7705 San Isabel Dr. Plano Texas US 75025 "#,
            0x8951 => r#"Werinherstrasse 91 München Bavaria DE D-81541 "#,
            0x8953 => r#"Leväsentie 23 Kuopio Pohjois-Savo FI 70780 "#,
            0x8954 => r#"5 Tavistock Estate Twyford Reading GB RG10 9NJ "#,
            0x8955 => r#"340 S. Lemon Ave Walnut CA US 91789 "#,
            0x8956 => r#"6-28-8 Shinjuku Shinjuku-ku Tokyo JP 1600022 "#,
            0x8BC2 => r#"Elektroniikkatie 8 Oulu  FI 90590 "#,
            0x9040..=0x905F => r#"B1, No. 9, Lane 50, Section 3, Taipei  TW  "#,
            0x909B => {
                r#"Gehua Building A1105, Dong Cheng District, Beijing Beijing Beijing CN 100007 "#
            }
            0x9433 => r#"Buschkauler Weg 27 Alfter  DE 53347 "#,
            0x96B6 | 0xA85A | 0xCCB1 => {
                r#" Bantian, Longgang District, Shenzhen, 518129, P.R.C Shenzhen GUANGDONG Province CN 518000 "#
            }
            0x99FE => r#"Leopoldstraße 236 Munich  DE 80807 "#,
            0x9A22 => r#"c/o Internet Society Reston MA US 20190-5108 "#,
            0x9AC6 => r#"3500 Deer Creek Rd. PALO ALTO CA US 94304 "#,
            0x9C40 => r#"Felix-Wankel-Straße 2 Ostfildern Baden-Württemberg DE 73760 "#,
            0xA8C8 => r#"445 Hoes Lane  Piscataway NJ US 08854-4141 "#,
            0xAAC5 => r#"1 Technology Way Norwood MA US MA02062 "#,
            0xAB37 | 0xED3E => r#"5177 Brandin Court Fremont CA US 94538 "#,
            0xAD3F => r#"1 Willowbrook Court, Suite 150 Petaluma CA US 94954 "#,
            0xAEFE => r#"Torshamnsgatan 21 Stockholm  SE 164 40 "#,
            0xB081 => {
                r#"Inovance Headquarters Tower, High-tech Industrial Park,Guanlan Street, Longhua New District, Shenzhen P.R.China Shenzhen Guangdong CN 518110 "#
            }
            0xB298 => r#"4333 Boulevard de la Grande-Allée Boisbriand Québec CA J7H 1M7 "#,
            0xB4E3 => r#"858 Coal Creek Circle Louisville CO US 80027 "#,
            0xB62C => r#"16101 N. 82nd Street, Suite 3B Scottsdale AZ US 85260-1868 "#,
            0xB6FE => r#"Ostendstr. 196 NUremberg  DE 90482 "#,
            0xB7EA | 0xEADD => r#"No 156 Beiqing Road, Haidian District Beijing  CN 100095 "#,
            0xBC17 => r#"94 Inverness Terrace E Englewood CO US 80111 "#,
            0xBC19 => r#"Level 15, Aldar HQ Abu Dhabi  AE 27655 "#,
            0xC111 => r#"6500 Kaiser Dr Suite 110 Fremont CA US 94555 "#,
            0xCCE0 => r#"208 84th Street 08242 Sea Isle City NJ US 08243 "#,
            0xCEB4 => r#"3300 General Motors Rd Milford MI US 48380 "#,
            0xD28B => r#"5453 Great America Parkway Santa Clara CA US 95054 "#,
            0xD672 => r#"1 Sansome Street, FL35 San Francisco CA US 94104 "#,
            0xDC94 => r#"Kartanontie 1 Helsinki  FI 00330 "#,
            0xE890 => r#"6205 Kestrel Road Mississauga Ontario CA L5T 2A1 "#,
            0xEC19 => {
                r#"Mithat Ulu Ünlü Sokak No. 23  Esentepe, Şişli  ISTANBUL ISTANBUL TR 34394 "#
            }
            0xF45F => {
                r#"2-2180,Building D,Block 33, No.99,Kechuang 14 Street,Beijing Economic-Technological Development Area Beijing  CN 100176 "#
            }
            0xF4C4 => r#"Koenigswarterstrasse 44 Fuerth Bavaria DE 90762 "#,
            0xF68E => r#"2-1 Kurosakishiroishi, Yahatanishi-ku, Kitakyushu  JP 806-0004 "#,
            0xF8DB => r#"Zugerstrasse 13 Ebikon  CH 6030 "#,
            0xFC0F => r#"Level 1 / 11 Queens Road Melbourne VICTORIA AU 3004 "#,
            0xFC3D => r#"445 Hoes Lane Piscataway NJ US 08854-4141 "#,
            0xFEA0..=0xFEAF => r#"1-14-5 Kichijoji-Honmachi Musashino-shi Tokyo JP 180 "#,
            _ => return None,
        })
    }
}
