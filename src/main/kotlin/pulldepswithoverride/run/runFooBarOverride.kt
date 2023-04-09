/*
 *  Copyright © 2022 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package pulldepswithoverride.run

import pulldepswithoverride.fs.*

fun main() {
	fooSflCfgSrc = { FooSflCfgInfo("Overridden fooSflCfgSrc") }

	fooSflDeps = FooSflDeps(
		barBf = { println("Overridden barBf") },
	)

	barBfCfgSrc = { BarBfCfgInfo(99) }

	fooSfl()
}
