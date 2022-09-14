/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

package tryout.moduleconfig.pullwithpushoverride.fs

import tryout.moduleconfig.pullwithpushoverride.config.AppCfgInfo

fun barBfCfgAdapter(appCfg: AppCfgInfo): BarBfCfgInfo {
	return BarBfCfgInfo(
		z = appCfg.y,
	)
}
