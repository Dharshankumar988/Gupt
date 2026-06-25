import SwiftUI
import Combine

class ChatViewModel: ObservableObject {
    // @Published var messages: [Message] = [] // using the generated UDL Message
    @Published var messages: [MessageModel] = []
    @Published var isLoading = false
    @Published var errorMessage: String? = nil
    
    let conversationId: String
    private var engine: GuptEngine?
    
    init(conversationId: String, engine: GuptEngine?) {
        self.conversationId = conversationId
        self.engine = engine
    }
    
    func loadMessages() {
        guard let engine = engine else { return }
        isLoading = true
        
        do {
            let ffiMessages = try engine.getMessages(conversationId: conversationId)
            // In reality, map ffiMessages to MessageModel here
            self.isLoading = false
        } catch {
            self.errorMessage = error.localizedDescription
            self.isLoading = false
        }
    }
    
    func sendMessage(_ text: String) {
        guard let engine = engine else { return }
        
        // Haptic feedback for send
        let generator = UIImpactFeedbackGenerator(style: .medium)
        generator.impactOccurred()
        
        do {
            let _ = try engine.sendMessage(conversationId: conversationId, content: text)
            // Reload messages...
        } catch {
            self.errorMessage = error.localizedDescription
        }
    }
}
