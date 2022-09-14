/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the Apache 2.0 license
 *  that can be found in the LICENSE file.
 */

package tryout.moduleconfig.pushtovar.run

import tryout.moduleconfig.pushtovar.fs.fooSfl
import tryout.moduleconfig.pushtovar.startup.initialize

fun main() {
	initialize()

	fooSfl()
}
