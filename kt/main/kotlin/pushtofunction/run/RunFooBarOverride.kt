/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package pushtofunction.run

import pushtofunction.config.getAppConfiguration
import pushtofunction.fs.BarBfCfgInfo
import pushtofunction.fs.FooSflCfgInfo
import pushtofunction.fs.boot.barBfCfgAdapter
import pushtofunction.fs.boot.fooSflBoot
import pushtofunction.fs.boot.fooSflCfgAdapter

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
