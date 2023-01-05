/*
 *  Copyright © 2022 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package pushtovar.startup

import pushtovar.config.getAppConfiguration
import pushtovar.fs.cfgadapt.barBfCfgAdaptation
import pushtovar.fs.cfgadapt.fooSflCfgAdaptation

fun initialize()  {
	val c = ::getAppConfiguration
	fooSflCfgAdaptation.setOrigin(c)
	barBfCfgAdaptation.setOrigin(c)
}
