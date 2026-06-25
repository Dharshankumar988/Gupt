import SwiftUI

struct OnboardingView: View {
    @State private var username: String = ""
    @State private var pin: String = ""
    
    // Simple state to simulate navigation after onboarding
    @Binding var isOnboardingComplete: Bool
    
    var body: some View {
        ZStack {
            // Background Gradient
            LinearGradient(
                gradient: Gradient(colors: [.guptBackgroundDark, .guptSurfaceDark]),
                startPoint: .top,
                endPoint: .bottom
            )
            .ignoresSafeArea()
            
            VStack(spacing: 24) {
                Spacer()
                
                Text("Welcome to Gupt")
                    .font(.system(size: 36, weight: .bold))
                    .foregroundColor(.guptTextPrimary)
                
                Text("Secure, hybrid communication via BLE, Wi-Fi Direct, Mesh, and Cloud.")
                    .font(.body)
                    .foregroundColor(.guptTextSecondary)
                    .multilineTextAlignment(.center)
                    .padding(.horizontal, 32)
                
                Spacer().frame(height: 32)
                
                VStack(spacing: 16) {
                    TextField("Choose a Username", text: $username)
                        .padding()
                        .background(Color.guptSurfaceElevated)
                        .cornerRadius(12)
                        .foregroundColor(.guptTextPrimary)
                        .preferredColorScheme(.dark)
                    
                    SecureField("Set a Secure PIN", text: $pin)
                        .padding()
                        .background(Color.guptSurfaceElevated)
                        .cornerRadius(12)
                        .foregroundColor(.guptTextPrimary)
                        .preferredColorScheme(.dark)
                        .keyboardType(.numberPad)
                }
                .padding(.horizontal, 24)
                
                Spacer().frame(height: 24)
                
                Button(action: {
                    // Trigger haptic feedback
                    let impactMed = UIImpactFeedbackGenerator(style: .medium)
                    impactMed.impactOccurred()
                    
                    // Simulate completion
                    withAnimation {
                        isOnboardingComplete = true
                    }
                }) {
                    Text("Generate Identity")
                        .font(.headline)
                        .foregroundColor(.white)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.guptPrimaryBlue)
                        .cornerRadius(16)
                }
                .padding(.horizontal, 24)
                
                Spacer()
            }
        }
    }
}

#Preview {
    OnboardingView(isOnboardingComplete: .constant(false))
}
