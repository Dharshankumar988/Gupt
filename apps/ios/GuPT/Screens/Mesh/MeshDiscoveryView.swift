import SwiftUI

struct DiscoveredPeer: Identifiable {
    let id: String
    let name: String
    let transport: String
    let trustScore: Double
}

struct MeshDiscoveryView: View {
    @State private var isScanning = true
    @State private var scale: CGFloat = 0.5
    @State private var opacity: Double = 1.0
    
    // Mock peers
    let peers = [
        DiscoveredPeer(id: "1", name: "Bob's Phone", transport: "BLE", trustScore: 0.8),
        DiscoveredPeer(id: "2", name: "Alice (Laptop)", transport: "Wi-Fi Direct", trustScore: 0.95),
        DiscoveredPeer(id: "3", name: "Unknown Device", transport: "BLE", trustScore: 0.2)
    ]
    
    var body: some View {
        NavigationView {
            ZStack {
                Color.guptBackgroundDark.ignoresSafeArea()
                
                VStack(spacing: 32) {
                    Spacer().frame(height: 16)
                    
                    // Radar Animation
                    ZStack {
                        Circle()
                            .stroke(Color.guptAccentTeal.opacity(opacity), lineWidth: 4)
                            .scaleEffect(scale)
                            .frame(width: 200, height: 200)
                            .onAppear {
                                withAnimation(Animation.easeOut(duration: 2.0).repeatForever(autoreverses: false)) {
                                    scale = 1.5
                                    opacity = 0.0
                                }
                            }
                        
                        Circle()
                            .fill(Color.guptAccentTeal)
                            .frame(width: 24, height: 24)
                        
                        Text("Scanning...")
                            .font(.caption)
                            .foregroundColor(.guptAccentTeal)
                            .offset(y: 40)
                    }
                    .frame(height: 250)
                    
                    VStack(alignment: .leading, spacing: 16) {
                        Text("Nearby Peers")
                            .font(.title2.bold())
                            .foregroundColor(.guptTextPrimary)
                            .padding(.horizontal, 24)
                        
                        ScrollView {
                            LazyVStack(spacing: 12) {
                                ForEach(peers) { peer in
                                    PeerCardView(peer: peer)
                                }
                            }
                            .padding(.horizontal, 20)
                        }
                    }
                }
            }
            .navigationTitle("Mesh Discovery")
            .navigationBarTitleDisplayMode(.inline)
            .toolbarColorScheme(.dark, for: .navigationBar)
            .toolbarBackground(Color.guptSurfaceDark, for: .navigationBar)
            .toolbarBackground(.visible, for: .navigationBar)
        }
    }
}

struct PeerCardView: View {
    let peer: DiscoveredPeer
    
    var trustColor: Color {
        if peer.trustScore > 0.8 { return .guptStatusSuccess }
        if peer.trustScore > 0.4 { return .guptStatusWarning }
        return .guptStatusError
    }
    
    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: 4) {
                Text(peer.name)
                    .font(.headline)
                    .foregroundColor(.guptTextPrimary)
                
                Text("Via \(peer.transport)")
                    .font(.subheadline)
                    .foregroundColor(.guptPrimaryBlue)
            }
            
            Spacer()
            
            VStack(alignment: .trailing, spacing: 2) {
                Text("Trust")
                    .font(.caption2)
                    .foregroundColor(.guptTextSecondary)
                
                Text("\(Int(peer.trustScore * 100))%")
                    .font(.headline)
                    .foregroundColor(trustColor)
            }
        }
        .padding()
        .background(Color.guptSurfaceElevated.opacity(0.6))
        .cornerRadius(16)
    }
}

#Preview {
    MeshDiscoveryView()
}
