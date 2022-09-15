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

func fooSflCfgAdapter(appCfgInfo config.AppCfgInfo) fs.FooSflCfgInfo {
	return fs.FooSflCfgInfo{
		X: appCfgInfo.X,
	}
}

var FooSflCfgAdapter = fwk.LiftToNullary(fooSflCfgAdapter)

func FooSflBoot(appCfgSrc config.AppCfgSrc) fs.FooSflT {
	return fs.FooSflC(fs.FooSflCfgSrc{
		Get:   FooSflCfgAdapter(appCfgSrc),
		BarBf: BarBfBoot(appCfgSrc),
	})
}
