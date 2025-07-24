//
//  EditUserForm.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 23/07/2025.
//

import SwiftUI

struct EditUserForm: View {
    @EnvironmentObject var editUserViewModel: EditUserViewModel
    
    var body: some View {
        Form {
            Section {
                Group {
                    if let error = editUserViewModel.error {
                        Text(LocalizedStringKey(error.localizationKey))
                            .foregroundStyle(.red)
                    }
                    
                    if let error = editUserViewModel.validationError {
                        Text(LocalizedStringKey(error.localizationKey))
                            .foregroundStyle(.red)
                    }
                }
                Group {
                    TextField("email", text: $editUserViewModel.user.email)
                        .autocapitalization(.none)
                        .keyboardType(.emailAddress)
                        .disableAutocorrection(true)
                    
                    Picker(selection: $editUserViewModel.locale, label: Text("lang")) {
                        ForEach(Bundle.main.localizations, id: \.self) { locale in
                            Text(locale).tag(locale)
                        }
                    }
                
                }
                
                Group {
                    EditUserEmailValidateButton()
                }
            }
            Section {
                Text("roles", comment: "The text in top of the roles picker")
                    .font(.headline)
                
                ForEach(editUserViewModel.roles, id: \.self) { role in
                    Toggle(role, isOn: Binding(
                        get: { editUserViewModel.user.roles.contains(role) },
                        set: { newValue in
                            if newValue && !editUserViewModel.user.roles.contains(role) {
                                editUserViewModel.user.roles.append(role)
                            } else if editUserViewModel.user.roles.contains(role) {
                                editUserViewModel.user.roles.removeAll { $0 == role }
                            }
                        }
                    ))
                }
                
                Group {
                    EditUserRolesValidateButton()
                }
            }
            .task {
                await editUserViewModel.getRoles()
            }
        }
    }
}

#Preview {
    EditUserForm()
        .environmentObject(EditUserViewModel())
}
