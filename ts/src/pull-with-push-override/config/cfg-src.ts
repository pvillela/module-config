import { ConfigurationError } from "../../arch/util/configuration-error";
import { AppCfgInfo, getAppConfiguration } from "./app-cfg";
import { makeMutableFunctionArity0, MutableFunctionArity0 } from "../../arch/util/mutable-function-arity-0";

export type CfgSrc<T> = MutableFunctionArity0<T>

function nilAdapter<T>(info: AppCfgInfo): T {
	throw new ConfigurationError("Module used before being initialized")
}

export function makeCfgSrc<T>(adapter: ((info: AppCfgInfo) => T) | null): CfgSrc<T> {
	const adapter1 = adapter || nilAdapter
	const initialSrc = () => adapter1(getAppConfiguration())
	return makeMutableFunctionArity0(initialSrc)
}

export function identity<T>(t: T): T {
	return t
}
