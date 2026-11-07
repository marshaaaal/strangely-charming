fn hook(packet: &ClientboundPlayPacket) {
    if let ClientboundPlayPacket::CustomPayload(p) = packet
        && p.channel == crate::game::sc_quests::BADGES
    {
        crate::game::sc_quests::read_badges(&p.data);
    }
}
