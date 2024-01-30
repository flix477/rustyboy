import Foundation
import SwiftData

@Model
final class Savestate {
    var timestamp: Date
    var game: Game
    
    @Attribute(.externalStorage)
    var image: Data
    
    @Attribute(.externalStorage)
    var data: Data
    
    init(timestamp: Date, game: Game, image: Data, data: Data) {
        self.timestamp = timestamp
        self.game = game
        self.image = image
        self.data = data
    }
}
