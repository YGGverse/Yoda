<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu\File;

class Open
{
    public \GtkMenuItem $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Menu\File $file;

    // Defaults
    private string $_label = 'Open';

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Menu\File $file
    ) {
        // Init dependencies
        $this->file = $file;

        // Init menu item
        $this->gtk = \GtkMenuItem::new_with_label(
            $this->_label
        );

        // Render
        $this->gtk->show();

        // Init events
        $this->gtk->connect(
            'activate',
            function()
            {
                $dialog = new \GtkFileChooserDialog(
                    'Open file',
                    $this->file->menu->browser->gtk,
                    \GtkFileChooserAction::OPEN,
                    [
                        'Cancel',
                        \GtkResponseType::CANCEL,
                        'Open',
                        \GtkResponseType::OK
                    ]
                );

                /* @TODO keep last path
                $dialog->set_current_folder();*/

                if (\GtkResponseType::OK == $dialog->run())
                {
                    $this->file->menu->browser->container->tab->append(
                        sprintf(
                            'file://%s',
                            $dialog->get_filename()
                        )
                    );
                }

                $dialog->destroy();
            }
        );
    }
}