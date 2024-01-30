import Foundation
import SwiftData

@Model
final class Game {
    var name: String
    var importDate: Date
    var lastPlayedDate: Date?
    
    @Attribute(.externalStorage)
    var rom: Data
    
    @Relationship(deleteRule: .cascade, inverse: \Savestate.game)
    var savestates: [Savestate]
    
    init(name: String, 
         rom: Data,
         importDate: Date,
         lastPlayedDate: Date? = nil,
         savestates: [Savestate] = []) {
        self.name = name
        self.rom = rom
        self.importDate = importDate
        self.lastPlayedDate = lastPlayedDate
        self.savestates = savestates
    }
}
