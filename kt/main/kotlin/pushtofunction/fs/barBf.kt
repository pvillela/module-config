/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package pushtofunction.fs

data class BarBfCfgInfo(
	val z: Int
)

data class BarBfCfgSrc(
	val get: () -> BarBfCfgInfo
)

typealias BarBfT = () -> Unit

fun barBfC(cfg: BarBfCfgSrc): BarBfT {
	return { println(cfg.get().z) }
}
