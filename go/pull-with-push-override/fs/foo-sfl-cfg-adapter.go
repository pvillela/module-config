/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

package fs

import (
	"github.com/pvillela/module-config/go/pull-with-push-override/config"
)

func FooSflCfgAdapter(appCfg config.AppCfgInfo) FooSflCfgInfo {
	return FooSflCfgInfo{
		X: appCfg.X,
	}
}
