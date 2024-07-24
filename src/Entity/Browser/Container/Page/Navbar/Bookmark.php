<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Navbar;

use \Yggverse\Yoda\Abstract\Entity\Browser\Container\Page\Navbar\Button;

class Bookmark extends Button
{
    private const _IMAGE_STARRED_YES = 'starred-symbolic';
    private const _IMAGE_STARRED_NON = 'non-starred-symbolic';

    public const IMAGE = self::_IMAGE_STARRED_NON;
    public const LABEL = 'Bookmark';
    public const TOOLTIP = 'Toggle bookmark';
    public const SENSITIVE = true;

    protected function _onCLick(
        \GtkButton $entity
    ): void
    {
        $this->setImage(
            $this->navbar->page->container->browser->database->toggleBookmark(
                $this->navbar->request->getValue()
            ) ? self::_IMAGE_STARRED_YES : self::_IMAGE_STARRED_NON
        );
    }

    public function refresh(): void
    {
        $this->setImage(
            $this->navbar->page->container->browser->database->getBookmark(
                $this->navbar->request->getValue()
            ) ? self::_IMAGE_STARRED_YES : self::_IMAGE_STARRED_NON
        );
    }
}