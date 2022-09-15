/*
 *  Copyright © 2022 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package pushtofunction.fs

data class FooSflCfgInfo(
	val x: String
)

data class FooSflCfgSrc(
	val get: () -> FooSflCfgInfo,
	val bar: BarBfT
)

typealias FooSflT = () -> Unit

fun fooSflC(cfg: FooSflCfgSrc): FooSflT {
	val barBf = cfg.bar
	return {
		println(cfg.get().x)
		barBf()
	}
}
