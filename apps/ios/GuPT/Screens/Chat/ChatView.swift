import SwiftUI

enum TransportRoute {
    case ble, wifiDirect, cloud, mesh
    
    var icon: String {
        switch self {
        case .ble: return "ᛒ" // Rune for Bluetooth
        case .wifiDirect: return "🛜"
        case .cloud: return "☁️"
        case .mesh: return "🕸️"
        }
    }
}

struct MessageModel: Identifiable {
    let id: String
    let text: String
    let isFromMe: Bool
    let time: String
    let transportUsed: TransportRoute
}

struct ChatView: View {
    let conversationId: String
    let name: String
    
    @State private var inputText = ""
    
    // Mock messages
    @State private var messages = [
        MessageModel(id: "1", text: "Hey, are you nearby?", isFromMe: false, time: "10:40 AM", transportUsed: .cloud),
        MessageModel(id: "2", text: "Yeah, I'm just across the room.", isFromMe: true, time: "10:41 AM", transportUsed: .cloud),
        MessageModel(id: "3", text: "Switching to BLE for efficiency.", isFromMe: false, time: "10:41 AM", transportUsed: .ble),
        MessageModel(id: "4", text: "Perfect, connection is much faster now.", isFromMe: true, time: "10:42 AM", transportUsed: .ble)
    ]
    
    var body: some View {
        ZStack {
            Color.guptBackgroundDark.ignoresSafeArea()
            
            VStack(spacing: 0) {
                ScrollView {
                    LazyVStack(spacing: 12) {
                        ForEach(messages) { msg in
                            MessageBubbleView(message: msg)
                        }
                    }
                    .padding()
                }
                
                // Input Bar
                HStack(spacing: 12) {
                    TextField("Message...", text: $inputText)
                        .padding(12)
                        .background(Color.guptSurfaceElevated)
                        .cornerRadius(20)
                        .foregroundColor(.guptTextPrimary)
                    
                    Button(action: {
                        guard !inputText.isEmpty else { return }
                        // Haptic feedback
                        let impact = UIImpactFeedbackGenerator(style: .light)
                        impact.impactOccurred()
                        
                        // Send mock message
                        messages.append(MessageModel(id: UUID().uuidString, text: inputText, isFromMe: true, time: "Now", transportUsed: .mesh))
                        inputText = ""
                    }) {
                        Image(systemName: "arrow.up.circle.fill")
                            .resizable()
                            .frame(width: 36, height: 36)
                            .foregroundColor(inputText.isEmpty ? .guptTextSecondary : .guptPrimaryBlue)
                    }
                    .disabled(inputText.isEmpty)
                }
                .padding()
                .background(Color.guptSurfaceDark)
            }
        }
        .navigationTitle(name)
        .navigationBarTitleDisplayMode(.inline)
    }
}

struct MessageBubbleView: View {
    let message: MessageModel
    
    var body: some View {
        HStack {
            if message.isFromMe { Spacer() }
            
            VStack(alignment: message.isFromMe ? .trailing : .leading, spacing: 4) {
                Text(message.text)
                    .padding(.horizontal, 16)
                    .padding(.vertical, 10)
                    .background(message.isFromMe ? Color.guptPrimaryBlue : Color.guptSurfaceElevated)
                    .foregroundColor(message.isFromMe ? .white : .guptTextPrimary)
                    .clipShape(BubbleShape(isFromMe: message.isFromMe))
                
                HStack(spacing: 4) {
                    Text(message.time)
                        .font(.caption2)
                        .foregroundColor(.guptTextSecondary)
                    
                    Text(message.transportUsed.icon)
                        .font(.caption2)
                }
            }
            
            if !message.isFromMe { Spacer() }
        }
    }
}

struct BubbleShape: Shape {
    let isFromMe: Bool
    
    func path(in rect: CGRect) -> Path {
        let path = UIBezierPath(
            roundedRect: rect,
            byRoundingCorners: [
                .topLeft,
                .topRight,
                isFromMe ? .bottomLeft : .bottomRight
            ],
            cornerRadii: CGSize(width: 16, height: 16)
        )
        return Path(path.cgPath)
    }
}

#Preview {
    NavigationView {
        ChatView(conversationId: "1", name: "Alice")
    }
}
