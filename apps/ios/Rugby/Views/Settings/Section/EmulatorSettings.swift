//
//  EmulatorSettings.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-09-15.
//

import SwiftUI

struct EmulatorSettings: View {
    @Environment(Failure.self) private var err
    @Environment(Options.self) private var opt

    /// Present file importer.
    @State private var fileImport = false

    var body: some View {
        @Bindable var cfg = opt.data

        Form {
            Section {
                if let path = cfg.img,
                    let boot = try? Data(contentsOf: path)
                {
                    HStack {
                        Label {
                            VStack(alignment: .listRowSeparatorLeading) {
                                Text(path.lastPathComponent)
                                Text(boot.count.formatted(.byteCount(style: .file)))
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                            }
                        } icon: {
                            Image(systemName: "text.document")
                        }
                        Spacer()
                        Button(
                            "Delete",
                            systemImage: "xmark",
                            role: .destructive,
                        ) {
                            let url = withAnimation { cfg.img.take() }
                            guard let url else { return }
                            do {
                                // Remove boot ROM
                                try FileManager.default.removeItem(at: url)
                            } catch { err.log(error) }
                        }
                        .bold()
                        .imageScale(.small)
                        .labelStyle(.iconOnly)
                    }
                }
                Button("Import", systemImage: "plus") {
                    fileImport.toggle()
                }
            } header: {
                Label("Boot ROM", systemImage: "memorychip")
            } footer: {
                Text(
                    """
                    If you supply a boot ROM image, it will be used when \
                    initializing an emulator instance.
                    """
                )
            }
        }
        .navigationTitle("Emulator")
        .fileImporter(
            isPresented: $fileImport,
            allowedContentTypes: [.data],
            allowsMultipleSelection: false
        ) { result in
            // Extract files on success
            guard case .success(let files) = result,
                let url = files.first
            else {
                return
            }
            // Acquire access permission
            if !url.startAccessingSecurityScopedResource() {
                fatalError("failed to securely access path: “\(url)”")
            }
            defer {
                url.stopAccessingSecurityScopedResource()
            }
            // Filesystem operations
            let fs = FileManager.default
            // Remove previous image
            if let old = cfg.img.take() {
                try? fs.removeItem(at: old)
            }
            // Copy boot ROM image
            let img = Library.root.appending(path: url.lastPathComponent)
            do {
                try fs.copyItem(at: url, to: img)
            } catch { err.log(error) }
            // Store URL to image
            withAnimation {
                cfg.img = img
            }
        }
    }
}

#Preview {
    EmulatorSettings()
        .environment(Failure())
        .environment(Options())
}
