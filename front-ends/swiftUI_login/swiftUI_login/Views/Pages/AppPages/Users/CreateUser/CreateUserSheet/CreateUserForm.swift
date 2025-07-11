//
//  CreateUserForm.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 04/07/2025.
//

import SwiftUI

struct CreateUserForm: View {
    
    @EnvironmentObject var createUserViewModel: CreateUserViewModel
    
    var body: some View {
        Form {
            Section {
                Group {
                    if let error = createUserViewModel.error {
                        Text(LocalizedStringKey(error.localizationKey))
                            .foregroundStyle(.red)
                    }
                    
                    if let error = createUserViewModel.validationError {
                        Text(LocalizedStringKey(error.localizationKey))
                            .foregroundStyle(.red)
                    }
                }
                Group {
                    TextField("email", text: $createUserViewModel.email)
                        .autocapitalization(.none)
                        .keyboardType(.emailAddress)
                        .disableAutocorrection(true)
                    
                    Picker(selection: $createUserViewModel.locale, label: Text("lang")) {
                        ForEach(Bundle.main.localizations, id: \.self) { locale in
                            Text(locale).tag(locale)
                        }
                    }
                }
            }
            Section {
                Text("roles", comment: "The text in top of the roles picker")
                    .font(.headline)
                
                ForEach(createUserViewModel.roles.keys.sorted(), id: \.self) { key in
                    Toggle(key, isOn: Binding(
                        get: { createUserViewModel.roles[key] ?? false },
                        set: { newValue in createUserViewModel.roles[key] = newValue }
                    ))
                }
            }
            .task {
                await createUserViewModel.getRoles()
            }
        }
    }
}

#Preview {
    CreateUserForm()
        .environmentObject(CreateUserViewModel())
}
