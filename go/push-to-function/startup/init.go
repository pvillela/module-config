/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package startup

import (
	"github.com/pvillela/module-config/go/push-to-function/config"
	"github.com/pvillela/module-config/go/push-to-function/fs/boot"
)

var FooSfl = boot.FooSflBoot(config.GetAppConfiguration)
