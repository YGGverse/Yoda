<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu\File;

use \Yggverse\Yoda\Entity\Browser\Menu\File;

class Open
{
    public \GtkMenuItem $gtk;

    // Dependencies
    public File $file;

    // Defaults
    public const LABEL = 'Open';
    public const MULTIPLE = true;
    public const PATTERN = [
        // pattern:name
        '*'     => 'All',
        '*.gmi' => null
    ];

    public function __construct(
        File $file
    ) {
        // Init dependencies
        $this->file = $file;

        // Init menu item
        $this->gtk = \GtkMenuItem::new_with_label(
            $this::LABEL
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

                if ($home = getenv('HOME')) // @TODO keep last path
                {
                    $dialog->set_current_folder(
                        $home
                    );
                }

                $dialog->set_select_multiple(
                    $this::MULTIPLE
                );

                foreach ($this::PATTERN as $pattern => $name)
                {
                    $filter = new \GtkFileFilter;

                    $filter->set_name(
                        $name ? $name : $pattern
                    );

                    $filter->add_pattern(
                        $pattern
                    );

                    $dialog->add_filter(
                        $filter
                    );
                }

                if (\GtkResponseType::OK == $dialog->run())
                {
                    foreach ($dialog->get_filenames() as $filename)
                    {
                        $this->file->menu->browser->container->tab->append(
                            sprintf(
                                'file://%s',
                                $filename
                            )
                        );
                    }
                }

                $dialog->destroy();
            }
        );
    }
}