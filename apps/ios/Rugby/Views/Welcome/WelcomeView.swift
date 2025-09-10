//
//  WelcomeView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-14.
//

import SwiftUI

struct WelcomeView: View {
    @Environment(\.dismiss) private var dismiss

    private var pages: [AnyView] = [
        AnyView(WelcomeMsg()),
        AnyView(WelcomePal()),
        AnyView(WelcomeEnd()),
    ]

    @State private var path = NavigationPath()

    var body: some View {
        NavigationStack(path: $path) {
            Group {
                pages[0]
                    .navigationDestination(for: Int.self) { index in
                        pages[index]
                            .toolbar(.hidden, for: .navigationBar)
                            .safeAreaPadding(.vertical, 48)
                    }
            }
            .safeAreaPadding(.vertical, 48)
        }
        .safeAreaInset(edge: .bottom) {
            GlassEffectContainer {
                HStack {
                    if !path.isEmpty {
                        Button {
                            withAnimation {
                                path.removeLast()
                            }
                        } label: {
                            Label("Back", systemImage: "chevron.backward")
                                .imageScale(.large)
                                .labelStyle(.iconOnly)
                        }
                        .buttonStyle(.glass)
                    }
                    Button {
                        let next = path.count + 1
                        if next < pages.count {
                            path.append(next)
                        } else {
                            dismiss()
                        }
                    } label: {
                        Text(path.count + 1 < pages.count ? "Continue" : "Finish")
                            .bold()
                            .frame(maxWidth: .infinity)
                    }
                    .buttonStyle(.glassProminent)
                }
                .controlSize(.large)
            }
        }
        .safeAreaPadding(.horizontal, 24)
    }
}

#Preview {
    WelcomeView()
        .environment(Options())
}
