import { fooSfl, fooSflCfgSrc } from "../fs/foo-sfl";
import { barBfCfgSrc } from "../fs/bar-bf";
import { thunkOf } from "../../../arch/util/functions";

fooSflCfgSrc.set(thunkOf({ x: "foo" }))
barBfCfgSrc.set(thunkOf({ z: 99 }))

fooSfl()
