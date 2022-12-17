import { Backend } from '../../operations/backend';
import {
  installLatest,
  switchToBeta,
  switchToNightly,
} from '../../operations/install';
import verify from '../../operations/verify';
import { Progress } from 'nx-request-api';
import { FocusButton } from '../buttons/focus_button';
import { MenuType } from '../menu';
import { AbstractMenu } from './abstract_menu';

/**
 * builds the menu that appears when HDR is not installed
 * @returns the "not installed" menu
 */
export default class NotInstalledMenu extends AbstractMenu<{
  setInfo: (info: string) => void;
  switchTo: (menu: MenuType) => void;
}> {
  public constructor(props: {
    setInfo: (info: string) => void;
    switchTo: (menu: MenuType) => void;
    version: string;
  }) {
    super(props);
  }

  render(): JSX.Element {
    return (
      <div id="menu">
        <div className="main-menu">
          <FocusButton
            text="Play Vanilla&nbsp;"
            className={'main-buttons'}
            onClick={() => Backend.instance().play()}
            onFocus={() => this.props.setInfo('Play vanilla Ultimate')}
          />

          <FocusButton
            text="Install HDR&nbsp;"
            className={'main-buttons'}
            onClick={async () => {
              await installLatest((p: Progress) => this.showProgress(p))
                .then(() => verify((p: Progress) => this.showProgress(p)))
                .then(async () => {
                  if (Backend.isSwitch()) {
                    alert(
                      "HDR's files have been installed. Please enable hdr, hdr-assets, and hdr-stages when the Arcadia menu opens."
                    );
                    await Backend.instance().openModManager();
                  } else {
                    alert("HDR's files have been installed.");
                  }
                })
                .then(() => this.showMenu())
                .then(() => this.props.switchTo(MenuType.CheckingInstalled))
                .catch((e) => {
                  this.showMenu();
                  console.error('Error while installing latest HDR.');
                });
            }}
            onFocus={() =>
              this.props.setInfo('Install the latest version of HDR')
            }
          />
        </div>
        {super.render()}
      </div>
    );
  }
}