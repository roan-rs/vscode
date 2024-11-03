import path from "path";
import {
	CancellationToken,
	commands,
	EventEmitter,
	ExtensionContext,
	InlayHint,
	InlayHintsProvider,
	languages,
	ProviderResult,
	Range,
	Selection,
	TextDocument,
	TextDocumentChangeEvent,
	TextEdit,
	Uri,
	window,
	workspace,
	WorkspaceEdit,
} from "vscode";

import {
	Disposable,
	Executable,
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient;
const outputChannel = window.createOutputChannel("Roan Extension");

export async function activate(context: ExtensionContext) {
	outputChannel.appendLine("Activating Roan Language Server extension...");

	// I know this is bad, but only for development
	const command =
		"C:\\dev\\roan\\roan-language\\server\\target\\release\\roan-language-server.exe";

	const run: Executable = {
		command,
		options: {
			env: {
				...process.env,
				RUST_LOG: "debug",
			},
		},
	};

	const serverOptions: ServerOptions = {
		run,
		debug: run,
	};
	let clientOptions: LanguageClientOptions = {
		documentSelector: [{ scheme: "file", language: "roan" }],
		synchronize: {
			fileEvents: workspace.createFileSystemWatcher("**/.clientrc"),
		},
	};

	client = new LanguageClient(
		"roan-language-server",
		"roan language server",
		serverOptions,
		clientOptions,
	);

	client.start();
	outputChannel.appendLine("Roan Language Server started.");
}

export function deactivate(): Thenable<void> | undefined {
	outputChannel.appendLine("Deactivating Roan Language Server extension...");
	if (!client) {
		return undefined;
	}
	return client.stop();
}
