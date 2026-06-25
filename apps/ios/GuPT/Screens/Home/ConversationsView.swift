import SwiftUI

struct ConversationPreview: Identifiable {
    let id: String
    let name: String
    let lastMessage: String
    let time: String
    let unreadCount: Int
}

struct ConversationsView: View {
    @State private var searchText = ""
    
    // Mock data
    let conversations = [
        ConversationPreview(id: "1", name: "Alice", lastMessage: "See you tomorrow!", time: "10:42 AM", unreadCount: 2),
        ConversationPreview(id: "2", name: "Bob (Mesh)", lastMessage: "Transfer complete.", time: "Yesterday", unreadCount: 0),
        ConversationPreview(id: "3", name: "Project Team", lastMessage: "Are we still on for the sync?", time: "Tuesday", unreadCount: 5)
    ]
    
    var filteredConversations: [ConversationPreview] {
        if searchText.isEmpty {
            return conversations
        } else {
            return conversations.filter { $0.name.localizedCaseInsensitiveContains(searchText) }
        }
    }
    
    var body: some View {
        NavigationView {
            ZStack {
                Color.guptBackgroundDark.ignoresSafeArea()
                
                List(filteredConversations) { conv in
                    NavigationLink(destination: ChatView(conversationId: conv.id, name: conv.name)) {
                        ConversationRow(conversation: conv)
                    }
                    .listRowBackground(Color.clear)
                    .listRowSeparator(.hidden)
                }
                .listStyle(.plain)
                .padding(.top, 8)
            }
            .navigationTitle("Gupt")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button(action: { /* New Chat */ }) {
                        Image(systemName: "square.and.pencil")
                            .foregroundColor(.guptPrimaryBlue)
                    }
                }
            }
            .toolbarBackground(Color.guptSurfaceDark, for: .navigationBar)
            .toolbarBackground(.visible, for: .navigationBar)
            .toolbarColorScheme(.dark, for: .navigationBar)
        }
        .searchable(text: $searchText, prompt: "Search securely...")
        .preferredColorScheme(.dark)
    }
}

struct ConversationRow: View {
    let conversation: ConversationPreview
    
    var body: some View {
        HStack(spacing: 16) {
            // Avatar Placeholder
            ZStack {
                Circle()
                    .fill(Color.guptSurfaceElevated)
                    .frame(width: 52, height: 52)
                
                Text(String(conversation.name.prefix(1)))
                    .font(.title3.bold())
                    .foregroundColor(.guptTextPrimary)
            }
            
            VStack(alignment: .leading, spacing: 4) {
                HStack {
                    Text(conversation.name)
                        .font(.headline)
                        .foregroundColor(.guptTextPrimary)
                    Spacer()
                    Text(conversation.time)
                        .font(.caption)
                        .foregroundColor(.guptTextSecondary)
                }
                
                HStack {
                    Text(conversation.lastMessage)
                        .font(.subheadline)
                        .foregroundColor(.guptTextSecondary)
                        .lineLimit(1)
                    
                    Spacer()
                    
                    if conversation.unreadCount > 0 {
                        Text("\(conversation.unreadCount)")
                            .font(.caption2.bold())
                            .foregroundColor(.white)
                            .padding(.horizontal, 6)
                            .padding(.vertical, 2)
                            .background(Color.guptPrimaryBlue)
                            .clipShape(Capsule())
                    }
                }
            }
        }
        .padding(.vertical, 8)
    }
}

#Preview {
    ConversationsView()
}
