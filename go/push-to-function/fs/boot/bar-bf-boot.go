/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

package boot

import (
	"github.com/pvillela/module-config/go/push-to-function/config"
	"github.com/pvillela/module-config/go/push-to-function/fs"
	"github.com/pvillela/module-config/go/push-to-function/fwk"
)

func barBfCfgAdapter(appCfgInfo config.AppCfgInfo) fs.BarBfCfgInfo {
	return fs.BarBfCfgInfo{
		Z: appCfgInfo.Y,
	}
}

var BarBfCfgAdapter = fwk.LiftToNullary(barBfCfgAdapter)

func BarBfBoot(appCfg config.AppCfgSrc) fs.BarBfT {
	return fs.BarBfC(fs.BarBfCfgSrc{
		Get: BarBfCfgAdapter(appCfg),
	})
}
