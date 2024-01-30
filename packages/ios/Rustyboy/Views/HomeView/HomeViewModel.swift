import Foundation
import Slingshot
import RustyboyCoreBindings
import OrderedCollections

struct HomeViewModel {
    let now: () -> Date
    
    enum ImportError: Error {
        case securityScopeError
        case ioError(Error)
        case invalidROM(CartridgeMetadataError)
        case fileImporterError(Error)
    }
    
    func didSelect(game: Game) {
        game.lastPlayedDate = now()
    }
    
    func group(games: [Game]) -> [(String, [Game])] {
        let now = now()
        let calendar = Calendar.current
        guard let today = calendar.date(bySettingHour: 0, minute: 0, second: 0, of: now),
              let yesterday = calendar.date(byAdding: .day, value: -1, to: today),
              let thisWeek = calendar.dateInterval(of: .weekOfYear, for: today)?.start,
              let lastWeek = calendar.date(byAdding: .day, value: -7, to: thisWeek),
              let thisMonth = calendar.dateInterval(of: .month, for: today)?.start,
              let lastMonth = calendar.date(byAdding: .month, value: -1, to: thisMonth),
              let lastThreeMonths = calendar.date(byAdding: .month, value: -2, to: lastMonth),
              let lastSixMonths = calendar.date(byAdding: .month, value: -3, to: lastThreeMonths),
              let thisYear = calendar.dateInterval(of: .year, for: today)?.start else { return [] }
        
        let staticIntervals = [(id: "today", condition: today),
                               (id: "yesterday", condition: yesterday),
                               (id: "this_week", condition: thisWeek),
                               (id: "last_week", condition: lastWeek),
                               (id: "this_month", condition: thisMonth),
                               (id: "last_month", condition: lastMonth),
                               (id: "last_three_months", condition: lastThreeMonths),
                               (id: "last_six_months", condition: lastSixMonths),
                               (id: "this_year", condition: thisYear)]
            .reduce { (acc: [(id: String, condition: Date)], x) in
                guard let last = acc.last else {
                    return [x]
                }
                
                if last.1 < x.1 {
                    return acc
                } else {
                    return acc + [x]
                }
            }
        
        let groups = (1...)
            .lazy
            .compactMap { calendar.date(byAdding: .year, value: -$0, to: thisYear) }
            .map { (id: "\(calendar.component(.year, from: $0))", condition: $0) }
            .prefix(by: staticIntervals)
        
        return groupc(orderedElements: games,
                      inOrderedGroups: groups,
                      byEvaluating: { (game: Game, intervalStart: Date) in
            (game.lastPlayedDate ?? game.importDate) >= intervalStart
        })
        .asArray
    }
    
    func importGames(atURLs urls: [URL]) -> Result<[Game], ImportError> {
        let importDate = now()
        return urls.map { url -> Result<Game, ImportError> in
            let name = url.lastPathComponent
            guard url.startAccessingSecurityScopedResource() else {
                return Result.failure(ImportError.securityScopeError)
            }
            
            defer { url.stopAccessingSecurityScopedResource() }
            
            do {
                let data = try Data(contentsOf: url)
                let _ = try RustyboyGameboy(buffer: data)
                return .success(Game(name: name, rom: data, importDate: importDate))
            } catch {
                if let error = error as? CartridgeMetadataError {
                    return .failure(.invalidROM(error))
                } else {
                    return .failure(.ioError(error))
                }
            }
        }.sequenced
    }
}
