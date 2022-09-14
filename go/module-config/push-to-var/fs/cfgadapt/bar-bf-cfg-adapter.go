/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

package cfgadapt

import (
	"github.com/pvillela/moduleconfig/go/module-config/push-to-var/config"
	"github.com/pvillela/moduleconfig/go/module-config/push-to-var/fs"
	"github.com/pvillela/moduleconfig/go/module-config/push-to-var/fwk"
)

func barBfCfgAdapter(appCfgInfo config.AppCfgInfo) fs.BarBfCfgInfo {
	return fs.BarBfCfgInfo{
		Z: appCfgInfo.Y,
	}
}

var BarBfCfgAdaptation = fwk.MakeCfgSrcAdaptation(
	&fs.BarBfCfgSrcV,
	barBfCfgAdapter,
)
