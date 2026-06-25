import SwiftUI

extension Color {
    // Backgrounds
    static let guptBackgroundDark = Color(hex: 0x0F172A)
    static let guptSurfaceDark = Color(hex: 0x1E293B)
    static let guptSurfaceElevated = Color(hex: 0x334155)
    
    // Accents
    static let guptPrimaryBlue = Color(hex: 0x3B82F6)
    static let guptPrimaryBlueDark = Color(hex: 0x2563EB)
    static let guptAccentTeal = Color(hex: 0x14B8A6)
    
    // Text
    static let guptTextPrimary = Color(hex: 0xF8FAFC)
    static let guptTextSecondary = Color(hex: 0x94A3B8)
    
    // Status
    static let guptStatusSuccess = Color(hex: 0x10B981)
    static let guptStatusWarning = Color(hex: 0xF59E0B)
    static let guptStatusError = Color(hex: 0xEF4444)
    
    init(hex: UInt, alpha: Double = 1) {
        self.init(
            .sRGB,
            red: Double((hex >> 16) & 0xff) / 255,
            green: Double((hex >> 08) & 0xff) / 255,
            blue: Double((hex >> 00) & 0xff) / 255,
            opacity: alpha
        )
    }
}
