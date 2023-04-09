/*
 *  Copyright © 2022 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package pulldepswithoverride.fs

import pulldepswithoverride.config.AppCfgInfo
import pulldepswithoverride.config.CfgSrc
import pulldepswithoverride.config.makeCfgSrc

data class FooSflCfgInfo(
	val x: String
)

data class FooSflDeps(
	val barBf: () -> Unit
)

var fooSflCfgSrc: CfgSrc<FooSflCfgInfo> =
//	makeCfgSrc(null) // this is used before fooSflCfgAdapter and/or the app config source are defined
	makeCfgSrc(::fooSflCfgAdapter) // replace above with this after fooSflCfgAdapter has been created

var fooSflDeps: FooSflDeps = FooSflDeps(
//	barBf = { TODO() } // do this at first, before barBf.kt exists
	barBf = ::barBf // replace above with this after barBf.kt has been created
)

fun fooSfl() {
	// Dependencies
	val barBf = fooSflDeps.barBf

	println(fooSflCfgSrc().x)
	barBf()
}

// This doesn't necessarily exist initially and may be added later, after the
// app configuration source has been created.
fun fooSflCfgAdapter(appCfg: AppCfgInfo): FooSflCfgInfo {
	return FooSflCfgInfo(
		x = appCfg.x,
	)
}
