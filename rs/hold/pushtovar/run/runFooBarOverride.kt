/*
 *  Copyright © 2022 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package pushtovar.run

import pushtovar.fs.BarBfCfgInfo
import pushtovar.fs.FooSflCfgInfo
import pushtovar.fs.barBfCfgSrc
import pushtovar.fs.fooSfl
import pushtovar.fs.fooSflCfgSrc

fun main() {
	fooSflCfgSrc.set { FooSflCfgInfo(x = "foo") }
	barBfCfgSrc.set { BarBfCfgInfo(z = 99) }

	fooSfl()
}
