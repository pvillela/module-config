/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the Apache 2.0 license
 *  that can be found in the LICENSE file.
 */

package tryout.moduleconfig.pushtovar.startup

import tryout.moduleconfig.pushtovar.config.getAppConfiguration
import tryout.moduleconfig.pushtovar.fs.cfgadapt.barBfCfgAdaptation
import tryout.moduleconfig.pushtovar.fs.cfgadapt.fooSflCfgAdaptation

fun initialize()  {
	val c = ::getAppConfiguration
	fooSflCfgAdaptation.setOrigin(c)
	barBfCfgAdaptation.setOrigin(c)
}
