//
//  LoginView.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 29/06/2025.
//

import SwiftUI

struct LoginView: View {
    @StateObject private var loginViewModel = LoginViewModel()
    
    @State private var loginError: String? = nil;
    @State private var email: String = "";
    @State private var password: String = "";
    
    var body: some View {
        VStack {
            ZStack {
                Text("login", comment: "The title of the login page")
                    .font(.title)
                    .fontWeight(.semibold)
                
                HStack {
                    Spacer()
                    
                    LoginValidateButton()
                        .environmentObject(loginViewModel)
                        .padding()
                }
                
            }
            
            
            Form {
                Group {
                    if loginViewModel.error != nil {
                        Text(loginViewModel.error!)
                            .foregroundStyle(Color(.red))
                    }
                }
                Group {
                    TextField("email", text: $loginViewModel.email)
                        .autocapitalization(.none)
                        .keyboardType(.emailAddress)
                        .disableAutocorrection(true)
                    
                    SecureField("password", text: $loginViewModel.password)
                        .autocapitalization(.none)
                        .disableAutocorrection(true)
                }
                
            }
        }
    }
}

#Preview {
    LoginView()
}
