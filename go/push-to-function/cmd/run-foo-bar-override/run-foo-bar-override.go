/*
 *  Copyright © 2022 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package main

import (
	"github.com/pvillela/module-config/go/arch/util"
	"github.com/pvillela/module-config/go/push-to-function/config"
	fs2 "github.com/pvillela/module-config/go/push-to-function/fs"
	boot2 "github.com/pvillela/module-config/go/push-to-function/fs/boot"
)

func main() {
	boot2.FooSflCfgAdapter = func(src config.AppCfgSrc) func() fs2.FooSflCfgInfo {
		return util.ThunkOf(fs2.FooSflCfgInfo{X: "foo"})
	}

	boot2.BarBfCfgAdapter = func(src config.AppCfgSrc) func() fs2.BarBfCfgInfo {
		return util.ThunkOf(fs2.BarBfCfgInfo{Z: 99})
	}

	{
		fooSfl := boot2.FooSflBoot(nil)
		fooSfl()
	}

	{
		fooSfl := boot2.FooSflBoot(config.GetAppConfiguration)
		fooSfl()
	}
}
