import SwiftUI

@main
struct GuPTApp: App {
    @StateObject private var engineProvider = EngineProvider()
    @State private var isOnboardingComplete = false
    
    init() {
        print("GuPT Application starting...")
    }

    var body: some Scene {
        WindowGroup {
            if isOnboardingComplete {
                MainTabView()
                    .environmentObject(engineProvider)
            } else {
                OnboardingView(isOnboardingComplete: $isOnboardingComplete)
                    .environmentObject(engineProvider)
            }
        }
    }
}
