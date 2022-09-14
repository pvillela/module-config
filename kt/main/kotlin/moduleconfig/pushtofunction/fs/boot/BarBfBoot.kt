/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

package tryout.moduleconfig.pushtofunction.fs.boot

import tryout.moduleconfig.pushtofunction.fs.BarBfCfgInfo
import tryout.moduleconfig.pushtofunction.fs.BarBfCfgSrc
import tryout.moduleconfig.pushtofunction.fs.BarBfT
import tryout.moduleconfig.pushtofunction.fs.barBfC
import tryout.moduleconfig.pushtofunction.config.AppCfgInfo
import tryout.moduleconfig.pushtovar.fwk.liftToNullary

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
