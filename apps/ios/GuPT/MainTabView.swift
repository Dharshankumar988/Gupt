import SwiftUI

struct MainTabView: View {
    @State private var selectedTab = 0
    
    var body: some View {
        TabView(selection: $selectedTab) {
            ConversationsView()
                .tabItem {
                    Label("Chats", systemImage: "message.fill")
                }
                .tag(0)
            
            MeshDiscoveryView()
                .tabItem {
                    Label("Mesh Radar", systemImage: "antenna.radiowaves.left.and.right")
                }
                .tag(1)
            
            Text("Settings")
                .foregroundColor(.guptTextPrimary)
                .frame(maxWidth: .infinity, maxHeight: .infinity)
                .background(Color.guptBackgroundDark.ignoresSafeArea())
                .tabItem {
                    Label("Settings", systemImage: "gear")
                }
                .tag(2)
        }
        .accentColor(.guptPrimaryBlue)
        .onAppear {
            let appearance = UITabBarAppearance()
            appearance.backgroundColor = UIColor(Color.guptSurfaceDark)
            appearance.unselectedItemTintColor = UIColor(Color.guptTextSecondary)
            
            UITabBar.appearance().standardAppearance = appearance
            if #available(iOS 15.0, *) {
                UITabBar.appearance().scrollEdgeAppearance = appearance
            }
        }
    }
}

#Preview {
    MainTabView()
}
