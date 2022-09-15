import { fooSflBoot, fooSflCfgAdapter } from "../fs/boot/foo-sfl-boot";
import { constantOf, thunkOf } from "../../arch/util/functions";
import { barBfCfgAdapter } from "../fs/boot/bar-bf-boot";
import { nilArity0 } from "../../arch/util/mutable-function-arity-0";
import { getAppConfiguration } from "../config/app-cfg";

function main() {
	fooSflCfgAdapter.set(constantOf(thunkOf({x: "foo" })))

	barBfCfgAdapter.set(constantOf(thunkOf({z: 99 })))

	{
		const fooSfl = fooSflBoot(nilArity0)
		fooSfl()
	}

	{
		const fooSfl = fooSflBoot(getAppConfiguration)
		fooSfl()
	}
}

main()
