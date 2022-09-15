/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

package pushtofunction.fs.boot

import pushtofunction.fs.FooSflCfgInfo
import pushtofunction.fs.FooSflCfgSrc
import pushtofunction.fs.FooSflT
import pushtofunction.fs.fooSflC
import pushtofunction.config.AppCfgInfo
import pushtovar.fwk.liftToNullary

private fun fooSflCfgAdapter0(appCfg: AppCfgInfo): FooSflCfgInfo {
	return FooSflCfgInfo(
		x = appCfg.x,
	)
}

var fooSflCfgAdapter = liftToNullary(::fooSflCfgAdapter0)

fun fooSflBoot(appCfg: (() -> AppCfgInfo)?): FooSflT {
	val fooSflCfgSrc = FooSflCfgSrc(
		get = fooSflCfgAdapter(appCfg),
		bar = barBfBoot(appCfg)
	)
	return fooSflC(fooSflCfgSrc)
}
