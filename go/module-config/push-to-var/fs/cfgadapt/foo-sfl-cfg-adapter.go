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

func fooSflCfgAdapter(appCfgInfo config.AppCfgInfo) fs.FooSflCfgInfo {
	return fs.FooSflCfgInfo{
		X: appCfgInfo.X,
	}
}

var FooSflCfgAdaptation = fwk.MakeCfgSrcAdaptation(
	&fs.FooSflCfgSrcV,
	fooSflCfgAdapter,
)
