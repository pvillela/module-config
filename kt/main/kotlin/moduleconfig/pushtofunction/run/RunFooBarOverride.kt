/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the Apache 2.0 license
 *  that can be found in the LICENSE file.
 */

package tryout.moduleconfig.pushtofunction.run

import tryout.moduleconfig.pushtofunction.config.getAppConfiguration
import tryout.moduleconfig.pushtofunction.fs.BarBfCfgInfo
import tryout.moduleconfig.pushtofunction.fs.FooSflCfgInfo
import tryout.moduleconfig.pushtofunction.fs.boot.barBfCfgAdapter
import tryout.moduleconfig.pushtofunction.fs.boot.fooSflBoot
import tryout.moduleconfig.pushtofunction.fs.boot.fooSflCfgAdapter

fun main() {
	fooSflCfgAdapter = { appSrc -> { FooSflCfgInfo(x = "foo") } }

	barBfCfgAdapter =  { appSrc -> { BarBfCfgInfo(z = 99) } }

	run {
		val fooSfl = fooSflBoot(null)
		fooSfl()
	}

	run {
		val fooSfl = fooSflBoot(::getAppConfiguration)
		fooSfl()
	}
}
