/*
 *  Copyright © 2022 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package pullwithpushoverride.fs

import pullwithpushoverride.config.CfgSrc
import pullwithpushoverride.config.makeCfgSrc

data class BarBfCfgInfo(
	val z: Int
)

var barBfCfgSrc: CfgSrc<BarBfCfgInfo> = makeCfgSrc(::barBfCfgAdapter)

fun barBf() {
	println(barBfCfgSrc().z)
}
