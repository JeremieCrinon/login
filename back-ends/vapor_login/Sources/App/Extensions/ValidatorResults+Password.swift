import Vapor

extension ValidatorResults {
    public struct Password {
        public let isValidPassword: Bool
    }
}

extension ValidatorResults.Password: ValidatorResult {
    public var isFailure: Bool {
        !self.isValidPassword
    }

    public var successDescription: String? {
        "is a valid password"
    }

    public var failureDescription: String? {
        "is not a valid password"
    }
}

let passwordRegex: String = "^(?=.*[a-z])(?=.*[A-Z])(?=.*\\d).{8,63}$"

extension Validator where T == String {
    public static var password: Validator<T> {
        .init { input in
            guard let range = input.range(of: passwordRegex, options: [.regularExpression]),
                  range.lowerBound == input.startIndex && range.upperBound == input.endIndex
            else {
                return ValidatorResults.Password(isValidPassword: false)
            }
            return ValidatorResults.Password(isValidPassword: true)
        }
    }
}
