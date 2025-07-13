//
//  UserRow.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 13/07/2025.
//

import SwiftUI

struct UserRow: View {
    var user: UserShort
    
    var body: some View {
        HStack {
            Text("\(user.id)")
            Text(user.email)
            Text(user.roles.joined(separator: " "))
        }
    }
}

#Preview {
    UserRow(user: UserShort(id: 1, email: "email@mail.com", roles: ["test", "vvv"]))
}
