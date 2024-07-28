<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu\File;

use \GtkMenuItem;
use \GtkFileChooserDialog;
use \GtkFileChooserAction;
use \GtkResponseType;

use \Yggverse\Yoda\Entity\Browser\Menu\File;

class Save
{
    // GTK
    public GtkMenuItem $gtk;

    // Dependencies
    public File $file;

    // Defaults
    public const LABEL = 'Save As..';

    public const DIALOG_TITLE = 'Save to file';
    public const DIALOG_BUTTON_CANCEL = 'Cancel';
    public const DIALOG_BUTTON_SAVE = 'Save';

    public function __construct(
        File $file
    ) {
        // Init dependencies
        $this->file = $file;

        // Init menu item
        $this->gtk = GtkMenuItem::new_with_label(
            $this::LABEL
        );

        // Render
        $this->gtk->show();

        // Init events
        $this->gtk->connect(
            'activate',
            function()
            {
                $dialog = new GtkFileChooserDialog(
                    _($this::DIALOG_TITLE),
                    $this->file->menu->browser->gtk,
                    GtkFileChooserAction::SAVE,
                    [
                        _($this::DIALOG_BUTTON_CANCEL),
                        GtkResponseType::CANCEL,
                        _($this::DIALOG_BUTTON_SAVE),
                        GtkResponseType::APPLY
                    ]
                );

                if ($home = getenv('HOME')) // @TODO keep last path
                {
                    $dialog->set_current_folder(
                        $home
                    );
                }

                $dialog->set_create_folders(
                    true
                );

                if (GtkResponseType::APPLY == $dialog->run())
                {
                    if ($page = $this->file->menu->browser->container->tab->get())
                    {
                        file_put_contents(
                            $dialog->get_filename(),
                            $page->content->getSource()
                        );
                    }
                }

                $dialog->destroy();
            }
        );
    }
}