import { fooSflCfgSrc } from "../foo-sfl";
import { CfgSrcAdaptation } from "../../fwk/cfg-src";
import { identity } from "../../../pull-with-push-override/config/cfg-src";

export const fooSflCfgAdaptation = new CfgSrcAdaptation(
	fooSflCfgSrc,
	identity,
)
