/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

package pushtofunction.fs.boot

import pushtofunction.fs.BarBfCfgInfo
import pushtofunction.fs.BarBfCfgSrc
import pushtofunction.fs.BarBfT
import pushtofunction.fs.barBfC
import pushtofunction.config.AppCfgInfo
import pushtovar.fwk.liftToNullary

private fun barBfCfgAdapter0(appCfg: AppCfgInfo): BarBfCfgInfo {
	return BarBfCfgInfo(
		z = appCfg.y,
	)
}

var barBfCfgAdapter = liftToNullary(::barBfCfgAdapter0)

fun barBfBoot(appCfg: (() -> AppCfgInfo)?): BarBfT {
	val barBfCfgSrc = BarBfCfgSrc(
		get = barBfCfgAdapter(appCfg)
	)
	return barBfC(barBfCfgSrc)
}
