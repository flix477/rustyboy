import SwiftUI

struct GameListCellView: View {
    let name: String
    let didSelect: () -> Void
    
    var body: some View {
        VStack(spacing: 6) {
            Image(.cartridge)
                .resizable()
                .frame(width: 160, height: 160)
                .shadow(color: .black.opacity(0.3), radius: 3)
            
            Text(name)
                .font(.semiBold(14))
                .padding(.vertical, 8)
        }
        .contextMenu {
            Button(action: didSelect) {
                Text("play")
            }
            Button(action: didSelect) {
                Text("delete")
            }
        }
        .onTapGesture(perform: didSelect)
    }
}
