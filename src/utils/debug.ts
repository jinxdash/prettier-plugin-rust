import { clamp, color, getTerminalWidth, normPath } from "./common";

const cwd = // @ts-expect-error
	typeof process === "object" && typeof process?.cwd === "function" ? /* @__PURE__ */ normPath(/* @__PURE__ */ process.cwd() ?? "") : "";
function normPath_strip_cwd(filepath: string) {
	let normFilePath = normPath(filepath);
	return normFilePath.startsWith(cwd) ? normFilePath.slice(cwd.length + 1) : normFilePath;
}

type StackStyleFn = (callee: string, item: StackItem) => (str: string) => string;
interface Stack extends Array<StackItem> {
	message: string;
	style?: { callee?: StackStyleFn; url?: StackStyleFn } | undefined;
}

class StackLine {
	declare readonly raw: string;
	declare readonly callee: string;
	declare readonly filepath: string;
	declare readonly line: string;
	declare readonly col: string;
	declare readonly other: string;
	declare readonly url: string;
	constructor(raw: string) {
		({
			1: this.callee = "",
			2: this.filepath = "",
			3: this.line = "",
			4: this.col = "",
			5: this.other = "",
		} = (this.raw = raw).match(/at (?:(.+?)\s+\()?(?:(.+?):([0-9]+)(?::([0-9]+))?|([^)]+))\)?/) ?? ["", "", "", "", "", ""]);
		this.url = this.filepath //
			? normPath_strip_cwd(this.filepath) + (this.line && this.col && `:${this.line}:${this.col}`)
			: this.other === "native"
			? "<native>"
			: "";
	}
}

function getPrintWidth() {
	return clamp(0, getTerminalWidth(128), 200) - 4;
}

class StackItem extends StackLine {
	constructor(private readonly stack: Stack, readonly i: number, raw: string) {
		super(raw);
	}
	hidden = false;
	hide() {
		this.hidden = true;
		return this;
	}
	hideNext(n: number) {
		for (let i = 0; i < n; i++) this.at(i)?.hide();
	}
	hideWhileTrue(test: (line: StackItem) => boolean) {
		let line: StackItem | undefined = this;
		while (line && test(line)) line = line.hide().next();
	}
	at(relIndex: number) {
		return this.i + relIndex >= this.stack.length || this.i + relIndex < 0 ? undefined : this.stack[this.i + relIndex];
	}
	next() {
		return this.at(+1);
	}
	toString() {
		const url = this.url;
		const calleeColor = this.stack.style?.callee?.(this.callee, this) ?? color.cyan;
		const urlColor = this.stack.style?.url?.(url, this) ?? color.grey;
		return compose2Cols("    at " + calleeColor(this.callee), urlColor(url), getPrintWidth());
	}
}

// prettier-ignore
function createStack(message: string, Error_stack: string, style: Stack["style"]): Stack {
	for (var STACK: Stack = [] as any, i = 0, stack = Error_stack.split("\n").slice(2); i < stack.length; i++) STACK[i] = new StackItem(STACK, i, stack[i]);
	return (STACK.message = message), (STACK.style = style), STACK;
}

function composeStack(stack: Stack) {
	var hidden = 0;
	var str = stack.message;
	for (var item of stack) item.hidden ? ++hidden : (str += "\n" + item.toString());
	return str + (hidden > 0 ? "\n" + color.grey(compose2Cols("", `...filtered ${hidden} lines`, getPrintWidth())) : "");
}

export function get_caller_cmd(offset = 0) {
	const obj: { stack: string } = {} as any;
	Error.captureStackTrace(obj, get_caller_cmd);
	const lines = obj.stack.split("\n");
	return new StackLine(lines[1 + clamp(0, lines.length - 2, offset)]).url;
}

var Error_prepareStackTrace;
let replaced_default_prepareStackTrace = false;
function custom_prepareStackTrace(err, calls) {
	return (Error_prepareStackTrace?.(err, calls) ?? calls.join("\n"))?.replace(/file:\/\/\//g, "").replace(/\\\\?/g, "/") ?? calls;
}

export function overrideDefaultError(silent = false) {
	if (replaced_default_prepareStackTrace === (replaced_default_prepareStackTrace = true)) return;
	Error_prepareStackTrace = Error.prepareStackTrace ?? ((_, calls) => calls.join("\n"));
	Error.prepareStackTrace = custom_prepareStackTrace;
	if (!silent) console.log(color.grey(`[devtools] Replaced Error.prepareStackTrace at ${get_caller_cmd(1)}`));
}

export function createCustomError({
	message = "Unknown Error",
	editStack = (stack: StackItem[]) => {},
	style = undefined as Stack["style"],
	stackTraceLimit = 20,
}): Error {
	const _stackTraceLimit = Error.stackTraceLimit;
	const _prepareStackTrace = Error.prepareStackTrace;
	if (replaced_default_prepareStackTrace && _prepareStackTrace === custom_prepareStackTrace)
		Error.prepareStackTrace = Error_prepareStackTrace;

	Error.stackTraceLimit = stackTraceLimit;

	const _ctx: { stack: string } = {} as any;

	Error.captureStackTrace(_ctx, createCustomError);

	const stack = createStack(message, _ctx.stack, style);
	Error.prepareStackTrace = function (err, calls) {
		editStack(stack);
		return composeStack(stack);
	};

	const err = new Error(message); // @ts-expect-error (get) to trigger prepareStackTrace, (set) to prevent treeshaking
	err.stack = err.stack;

	Error.stackTraceLimit = _stackTraceLimit;
	Error.prepareStackTrace = _prepareStackTrace;

	return err;
}

function compose2Cols(left: string, right: string, len = 64, min = 1) {
	return left + " ".repeat(clamp(min, len, len - (color.unstyledLength(left) + color.unstyledLength(right)))) + right;
}
